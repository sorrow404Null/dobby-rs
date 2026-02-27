use super::{Backend, HookBuild};
use crate::arch::aarch64;
use crate::error::Result;
use crate::platform;
use core::ffi::{c_char, c_void};
use core::ptr;

pub(crate) static BACKEND: UnixAarch64 = UnixAarch64;
pub(crate) struct UnixAarch64;

impl UnixAarch64 {
    const JMP_STUB_SIZE: usize = 16;
    const PATCH_LEN: usize = 16;
    fn abs_jmp(dest: u64) -> [u8; 16] {
        let ldr_x17_lit_8: u32 = 0x58000000 | (2 << 5) | 17;
        let br_x17: u32 = 0xD61F0000 | (17 << 5);
        let mut buf = [0u8; 16];
        buf[0..4].copy_from_slice(&ldr_x17_lit_8.to_le_bytes());
        buf[4..8].copy_from_slice(&br_x17.to_le_bytes());
        buf[8..16].copy_from_slice(&dest.to_le_bytes());
        buf
    }
}

impl Backend for UnixAarch64 {
    unsafe fn code_patch(
        &self,
        address: *mut c_void,
        buffer: *const u8,
        size: usize,
    ) -> Result<()> {
        platform::unix::code_patch(address, buffer, size)
    }
    unsafe fn hook_build(&self, address: *mut c_void, fake_func: *mut c_void) -> Result<HookBuild> {
        let stolen = core::slice::from_raw_parts(address as *const u8, Self::PATCH_LEN);
        let mut words = [0u32; 4];
        for i in 0..4 {
            words[i] = u32::from_le_bytes([
                stolen[i * 4],
                stolen[i * 4 + 1],
                stolen[i * 4 + 2],
                stolen[i * 4 + 3],
            ]);
        }
        let relocated = aarch64::relocate(&words, address as u64, 0x1000)?;
        let tramp_size = 256usize;
        let tramp = platform::unix::alloc_executable(tramp_size)?;
        let mut offset = 0usize;
        for w in relocated {
            ptr::copy_nonoverlapping(w.to_le_bytes().as_ptr(), (tramp as *mut u8).add(offset), 4);
            offset += 4;
        }
        let jb = Self::abs_jmp(address as u64 + Self::PATCH_LEN as u64);
        ptr::copy_nonoverlapping(
            jb.as_ptr(),
            (tramp as *mut u8).add(offset),
            Self::JMP_STUB_SIZE,
        );
        platform::unix::flush_icache(tramp, offset + Self::JMP_STUB_SIZE);
        let detour = Self::abs_jmp(fake_func as u64);
        platform::unix::code_patch(address, detour.as_ptr(), detour.len())?;
        Ok(HookBuild {
            trampoline: tramp,
            trampoline_size: tramp_size,
            original: stolen.to_vec(),
            patch_len: Self::PATCH_LEN,
        })
    }
    unsafe fn hook_destroy(
        &self,
        address: *mut c_void,
        original: &[u8],
        patch_len: usize,
        trampoline: *mut c_void,
        trampoline_size: usize,
    ) -> Result<()> {
        platform::unix::restore_patch(address, original, patch_len)?;
        platform::unix::free_executable(trampoline, trampoline_size)
    }
    unsafe fn symbol_resolver(
        &self,
        image_name: *const c_char,
        symbol_name: *const c_char,
    ) -> *mut c_void {
        platform::unix::symbol_resolver(image_name, symbol_name)
    }
}
