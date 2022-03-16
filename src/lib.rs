//! This crate is a rusty binding of [Dobby](https://github.com/jmpews/Dobby).
//!
//! # Quickstart
//!
//! ```
//! use dobby_rs::{resolve_symbol, hook, Address};
//!
//! #[inline(never)]
//! #[no_mangle]
//! extern "C" fn add(a: u64, b: u64) -> u64 {
//!     a + b
//! }
//!
//! #[inline(never)]
//! #[no_mangle]
//! extern "C" fn sub(a: u64, b: u64) -> u64 {
//!     a - b
//! }
//!
//! let addr = add as usize as Address;
//! let replace = sub as usize as Address;
//! unsafe {
//!     hook(addr, replace).unwrap();
//! }
//! ```

use dobby_sys::ffi;
use std::ffi::{c_void, CString};

pub type Address = *mut c_void;

/// Locate a symbol name within an image.
/// This function will return `None` if the symbol does not exist or the image has not been loaded yet.
/// # Panics
/// Panic if `image_name` or `symbol_name` contains byte '\x00'
pub fn resolve_symbol(image_name: &str, symbol_name: &str) -> Option<Address> {
    let image_name = CString::new(image_name).unwrap();
    let symbol_name = CString::new(symbol_name).unwrap();

    let addr = unsafe { ffi::DobbySymbolResolver(image_name.as_ptr(), symbol_name.as_ptr()) };
    if addr.is_null() {
        None
    } else {
        Some(addr)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DobbyHookError {
    #[error("hook error")]
    HookError,
}

/// Set up a hook at `addr`, return the trampoline address of the original function.
/// # Safety
/// Hooking is NOT SAFE. Use with your own caution.
pub unsafe fn hook(addr: Address, replace: Address) -> Result<Address, DobbyHookError> {
    let mut origin = std::ptr::null_mut();
    hook_and_update_origin(addr, replace, &mut origin)?;
    Ok(origin)
}

/// Set up a hook at `addr`. The trampoline address will be set simultaneously.
/// # Safety
/// Hooking is NOT SAFE. Use with your own caution.
pub unsafe fn hook_and_update_origin(
    addr: Address,
    replace: Address,
    origin: &mut Address,
) -> Result<(), DobbyHookError> {
    let ret = ffi::DobbyHook(addr, replace, origin as *mut _);
    if ret == 0 {
        Ok(())
    } else {
        Err(DobbyHookError::HookError)
    }
}

/// Undo all hooks at `addr`.
/// # Safety
pub unsafe fn unhook(addr: Address) -> Result<(), DobbyHookError> {
    let ret = ffi::DobbyDestroy(addr);
    if ret == 0 {
        Ok(())
    } else {
        Err(DobbyHookError::HookError)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DobbyMemoryOperationError {
    #[error("memory operation error")]
    MemoryOperationError,
    #[error("not enough memory")]
    NotEnough,
    #[error("not support allocate executable memory")]
    NotSupportAllocateExecutableMemory,
    #[error("unknown error")]
    None,
}

/// Patch the code at `addr` with supplied bytes.
/// # Safety
/// Patching code is NOT SAFE. Use with your own caution.
pub unsafe fn patch_code(addr: Address, code: &[u8]) -> Result<(), DobbyMemoryOperationError> {
    let ret = ffi::CodePatch(addr, code.as_ptr() as *mut _, code.len() as u32);
    match ret {
        ffi::MemoryOperationError_kMemoryOperationSuccess => Ok(()),
        ffi::MemoryOperationError_kMemoryOperationError => {
            Err(DobbyMemoryOperationError::MemoryOperationError)
        }
        ffi::MemoryOperationError_kNotEnough => Err(DobbyMemoryOperationError::NotEnough),
        ffi::MemoryOperationError_kNotSupportAllocateExecutableMemory => {
            Err(DobbyMemoryOperationError::NotSupportAllocateExecutableMemory)
        }
        ffi::MemoryOperationError_kNone => Err(DobbyMemoryOperationError::None),
        _ => unreachable!(),
    }
}
