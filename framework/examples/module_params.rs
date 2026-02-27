//! EN: `ModuleHandle` + pointer helpers in `framework::params`.
//! CN: `ModuleHandle` + `framework::params` 的指针辅助函数。

mod common;

use core::ffi::c_void;
use dobby_rs_framework::framework::{ModuleHandle, params};
use std::ffi::CString;

#[inline(never)]
fn local_add(x: i32) -> i32 {
    x + 7
}

fn main() -> dobby_rs_framework::Result<()> {
    // EN: 1) `cast_fn` - turn a raw address into a typed function pointer.
    // CN: 1) `cast_fn` - 把地址转换成带类型的函数指针。
    let p = local_add as *const () as *mut c_void;
    let f: fn(i32) -> i32 = unsafe { params::cast_fn(p) };
    println!("cast_fn(local_add)(1) = {}", f(1));

    // EN: 2) `read_ptr_value` / `write_ptr_value` - simple raw memory access.
    // CN: 2) `read_ptr_value` / `write_ptr_value` - 简单的裸指针读写。
    let mut buf = [0u8; 8];
    let buf_ptr = buf.as_mut_ptr() as *mut c_void;
    unsafe {
        params::write_ptr_value(buf_ptr, 0x11223344u32);
        let v: u32 = params::read_ptr_value(buf_ptr as *const c_void);
        println!("read_ptr_value = 0x{v:08x}");
    }

    // EN: 3) `ModuleHandle` - open a module and resolve a symbol.
    // CN: 3) `ModuleHandle` - 打开动态库并解析符号。
    let m = ModuleHandle::open(common::DEMO_LIB)?;
    println!("opened module: {}", m.lib_name());

    // EN: Pick a tiny symbol per OS.
    // CN: 选择一个各平台都常见的小符号。
    #[cfg(windows)]
    let sym = "GetCurrentThreadId";
    #[cfg(unix)]
    let sym = "puts";

    let addr = m.wrapped_sym(sym);
    println!("resolved {sym} = {:?}", addr);

    // EN: `cast_ptr` is just a typed cast helper.
    // CN: `cast_ptr` 只是一个带类型的转换辅助。
    let _typed: *mut u8 = params::cast_ptr(buf_ptr);

    // EN: CString is a common helper when calling `resolve` directly.
    // CN: 如果直接用 `resolve`，通常要用 CString 来构造 CStr。
    let _ = CString::new(sym).ok();
    Ok(())
}
