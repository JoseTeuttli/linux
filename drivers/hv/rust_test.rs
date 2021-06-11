
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::bindings::*;
use kernel::c_types;
//extern crate libc;
const __LOG_PREFIX: &[u8] = b"rust_hv\0";
#[no_mangle]
pub extern "C" fn print_hello() -> () {
    pr_info!("Testing rust integration, testing, testing, 123");
}
#[no_mangle]
pub extern "C" fn rust_hv_get_next_write_location(ring_buffer_info: *mut hv_ring_buffer_info) -> c_types::c_uint {
    print_hello();
    unsafe {
    pr_info!("Trying to read from passed data structure got int: {}", (*(*ring_buffer_info).ring_buffer).write_index);
    (*(*ring_buffer_info).ring_buffer).write_index
    }
}