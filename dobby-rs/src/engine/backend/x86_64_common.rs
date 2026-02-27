use super::HookBuild;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::ptr;
use iced_x86::{
    BlockEncoder, BlockEncoderOptions, Decoder, DecoderOptions, Instruction, InstructionBlock,
};

pub(crate) trait X64HookPlatform {
    unsafe fn alloc_executable(size: usize) -> Result<*mut c_void>;
    unsafe fn free_executable(ptr: *mut c_void, size: usize) -> Result<()>;
    unsafe fn flush_icache(address: *mut c_void, size: usize) -> Result<()>;
    unsafe fn write_detour_with_nops(
        address: *mut c_void,
        stolen_len: usize,
        detour: &[u8; 14],
    ) -> Result<()>;
}

fn abs_jmp(dest: u64) -> [u8; 14] {
    let mut b = [0u8; 14];
    b[0] = 0xFF;
    b[1] = 0x25;
    b[6..14].copy_from_slice(&dest.to_le_bytes());
    b
}

pub(crate) unsafe fn hook_build<P: X64HookPlatform>(
    address: *mut c_void,
    fake_func: *mut c_void,
) -> Result<HookBuild> {
    let target_ip = address as u64;
    let bytes = core::slice::from_raw_parts(address as *const u8, 64);
    let mut decoder = Decoder::with_ip(64, bytes, target_ip, DecoderOptions::NONE);
    let mut insns: Vec<Instruction> = Vec::new();
    let mut stolen_len = 0usize;
    while stolen_len < 14 {
        let i = decoder.decode();
        if i.is_invalid() {
            return Err(Error::DecodeFailed);
        }
        stolen_len += i.len();
        insns.push(i);
    }
    let original = core::slice::from_raw_parts(address as *const u8, stolen_len).to_vec();
    let tramp_size = 256usize;
    let tramp = P::alloc_executable(tramp_size)?;
    let block = InstructionBlock::new(&insns, tramp as u64);
    let encoded = BlockEncoder::encode(64, block, BlockEncoderOptions::NONE)
        .map_err(|_| Error::EncodeFailed)?;
    let code = encoded.code_buffer;
    if code.len() + 14 > tramp_size {
        let _ = P::free_executable(tramp, tramp_size);
        return Err(Error::EncodeFailed);
    }
    ptr::copy_nonoverlapping(code.as_ptr(), tramp as *mut u8, code.len());
    let jmp_back = abs_jmp(target_ip + stolen_len as u64);
    ptr::copy_nonoverlapping(jmp_back.as_ptr(), (tramp as *mut u8).add(code.len()), 14);
    P::flush_icache(tramp, code.len() + 14)?;
    let detour = abs_jmp(fake_func as u64);
    P::write_detour_with_nops(address, stolen_len, &detour)?;
    P::flush_icache(address, stolen_len)?;
    Ok(HookBuild {
        trampoline: tramp,
        trampoline_size: tramp_size,
        original,
        patch_len: stolen_len,
    })
}
