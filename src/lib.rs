use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

mod copy_move;  // Assuming this module exports rust_copy and rust_move

pub use copy_move::{rust_copy, rust_move};

// C-compatible wrapper for rust_copy
#[unsafe(no_mangle)]  // Updated: Wrap in unsafe(...)
pub extern "C" fn rust_copy_c(src: *const c_char, dst: *const c_char) -> c_int
{
    // Convert C strings to Rust strings
    let src_str = unsafe
        {
        if src.is_null()
        {
            return -1; // Null pointer error
        }
        match CStr::from_ptr(src).to_str()
        {
            Ok(s) => s,
            Err(_) => return -2, // Invalid UTF-8 string
        }
    };
    let dst_str = unsafe
        {
            if dst.is_null()
            {
                return -1; // Null pointer error
            }
            match CStr::from_ptr(dst).to_str()
            {
                Ok(s) => s,
                Err(_) => return -2, // Invalid UTF-8 string
            }
        };

    // Call the Rust function (assuming rust_copy is imported/visible from mod copy_move)
    match rust_copy(src_str, dst_str)
    {
        Ok(()) => 0, // Success
        Err(e) =>
            {
                eprintln!("Error in rust_copy: {:?}", e);
                -3 // Generic error (you can define more specific codes)
            }
    }
}

// C-compatible wrapper for rust_move
#[unsafe(no_mangle)]  // Updated: Wrap in unsafe(...)
pub extern "C" fn rust_move_c(src: *const c_char, dst: *const c_char) -> c_int
{
    // Convert C strings to Rust strings
    let src_str = unsafe
        {
            if src.is_null() {
                return -1; // Null pointer error
            }
            match CStr::from_ptr(src).to_str()
            {
                Ok(s) => s,
                Err(_) => return -2, // Invalid UTF-8 string
            }
        };
    let dst_str = unsafe
        {
            if dst.is_null()
            {
                return -1; // Null pointer error
            }
            match CStr::from_ptr(dst).to_str() {
                Ok(s) => s,
                Err(_) => return -2, // Invalid UTF-8 string
            }
        };

    // Call the Rust function (assuming rust_move is imported/visible from mod copy_move)
    match rust_move(src_str, dst_str)
    {
        Ok(()) => 0, // Success
        Err(e) =>
            {
                eprintln!("Error in rust_move: {:?}", e);
                -3 // Generic error
            }
    }
}