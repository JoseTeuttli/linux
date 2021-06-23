#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::bindings::*;
use kernel::c_types;

// extern "C" {
//     fn cancel_delayed_work_sync(input: ) -> ();
// }
//srv: *mut hv_util_service

extern "C" {
    fn hv_fcopy_cancel_work() -> ();
    fn hvutil_transport_destroy(hvt: *mut hvutil_transport) -> ();
}
const __LOG_PREFIX: &[u8] = b"rust_hv\0";

// static mut fcopy_transaction: fcopy_transaction_struct = fcopy_transaction_struct {
//     state: 0,
//     recv_len: 0,
//     fcopy_msg: core::ptr::null::<hv_fcopy_hdr>() as *mut hv_fcopy_hdr,
//     //fcopy_msg: core::ptr::null(),
//     //fcopy_msg: core::ptr::null<hv_fcopy_hdr>(),
//     //fcopy_msg: core::ptr::null::() as *mut hv_fcopy_hdr,
//     recv_channel: core::ptr::null::<vmbus_channel>() as *mut vmbus_channel,
//     //recv_channel: core::ptr::null(),
//     //recv_channel: core::ptr::null<vmbus_channel>(),
//     //recv_channel: core::ptr::null() as *mut vmbus_channel,
//     recv_req_id: 0,
// };

// #[no_mangle]
// pub extern "C" fn hv_fcopy_init_rust(srv: *mut hv_util_service) -> c_types::c_uint {
//     let recv_buffer: *mut c_types::c_uchar; //this should be out at top but let can only be called within a function (?)
//     pr_info!("in hv_fcopy_init_rust");
//     unsafe {
//         recv_buffer = (*srv).recv_buffer;
//     }
//     unsafe {
//         //let null_ptr: *mut hv_fcopy_hdr = core::ptr::null() ;
//         let mut fcopy_transaction: fcopy_transaction_struct = fcopy_transaction_struct {
//             state: 0,
//             recv_len: 0,
//             fcopy_msg: core::ptr::null() as *mut hv_fcopy_hdr,
//             //fcopy_msg: bindings::krealloc(core::ptr::null(), )
//             //fcopy_msg: core::ptr::null() as *mut hv_fcopy_hdr,
//             recv_channel: core::ptr::null() as *mut vmbus_channel,
//             recv_req_id: 0,
//         };
//     // let mut fcopy_transaction: fcopy_transaction_struct = fcopy_transaction_struct { //needs to be initialized
//     //     state: 0,
//     //     recv_len: 0,
//     //     fcopy_msg: ,
//     //     recv_channel: ,
//     //     recv_req_id: 0,
//     // };
    
//         (fcopy_transaction).recv_channel = (*srv).channel as *mut vmbus_channel;
//         (fcopy_transaction).state = 0; //supposed to be = HVUTIL_DEVICE_INIT
//     }
//     0
// }

// #[no_mangle]
// pub extern "C" fn hv_fcopy_deinit_rust() -> () {
//     fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_DYING;
// }

#[no_mangle]
pub extern "C" fn hv_fcopy_deinit_rust(mut fcopy_transaction: fcopy_transaction_struct, hvt: *mut hvutil_transport) -> () {
    fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_DYING as c_types::c_int;
    unsafe {
        hv_fcopy_cancel_work();
        hvutil_transport_destroy(hvt);
    }
}