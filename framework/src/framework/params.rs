use core::ffi::c_void;

/// Cast a raw pointer (address) to a function pointer type.
///
/// # Safety
/// - `T` must be a function pointer type with the correct ABI/signature.
/// - The address must be valid to call as `T`.
pub unsafe fn cast_fn<T: Copy>(ptr: *mut c_void) -> T {
    debug_assert_eq!(
        core::mem::size_of::<T>(),
        core::mem::size_of::<*mut c_void>()
    );
    core::mem::transmute_copy(&ptr)
}

/// Cast a raw pointer to a typed pointer.
pub fn cast_ptr<T>(ptr: *mut c_void) -> *mut T {
    ptr as *mut T
}

/// Read a value from a raw pointer (unaligned-safe).
///
/// # Safety
/// - `ptr` must be valid for reads of `size_of::<T>()` bytes.
pub unsafe fn read_ptr_value<T: Copy>(ptr: *const c_void) -> T {
    core::ptr::read_unaligned(ptr as *const T)
}

/// Write a value to a raw pointer (unaligned-safe).
///
/// # Safety
/// - `ptr` must be valid for writes of `size_of::<T>()` bytes.
pub unsafe fn write_ptr_value<T>(ptr: *mut c_void, value: T) {
    core::ptr::write_unaligned(ptr as *mut T, value);
}
