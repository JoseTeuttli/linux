#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::bindings::*;
use kernel::c_types;

// extern "C" {
//     fn cancel_delayed_work_sync(input: ) -> ();
// }

#[no_mangle]
pub extern "C" fn hv_fcopy_init_rust(srv: *mut hv_util_service) -> () {

}