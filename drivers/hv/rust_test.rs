
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
//extern crate libc;

#[no_mangle]
pub extern "C" fn print_hello() -> () {
    pr_info!("Testing rust integration");
}