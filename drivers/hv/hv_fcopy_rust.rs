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
    fn tasklet_enable(t: *mut tasklet_struct) -> ();
    fn tasklet_schedule(t: *mut tasklet_struct) -> ();
    fn fcopy_respond_to_host(error: c_types::c_int) -> ();
    fn fcopy_poll_wrapper(channel: *mut c_types::c_void);
    //not sure how to properly type define hv_poll_channel's 2nd parameter
    //I think the cb struct is defined here: https://github.com/Rust-for-Linux/linux/blob/rust/drivers/net/ethernet/intel/e100.c#L474
    fn hv_poll_channel(channel: *mut vmbus_channel, x: unsafe extern "C" fn(channel: *mut c_types::c_void));
    static mut fcopy_timeout_work: delayed_work;
    static mut fcopy_send_work: work_struct;
    static mut fcopy_transaction: fcopy_transaction_struct;
}
const __LOG_PREFIX: &[u8] = b"rust_hv\0";
const fcopy_devname: &[u8] = b"vmbus/hv_fcopy\0";
// impl default trait
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

#[no_mangle]
pub extern "C" fn fcopy_poll_wrapper_rust(channel: *mut c_types::c_void) -> () {
    pr_info!("Running Rust fcopy_poll_wrapper_rust");
    unsafe {
        fcopy_transaction.state = hvutil_device_state_HVUTIL_READY as c_types::c_int;
        tasklet_schedule(&mut((*(channel as *mut vmbus_channel)).callback_event))
    }
}

#[no_mangle]
pub extern "C" fn fcopy_timeout_func_rust() -> () {
    pr_info!("Running Rust fcopy_timeout_func_rust");
    unsafe {
        fcopy_respond_to_host(HV_E_FAIL as c_types::c_int);
        hv_poll_channel(fcopy_transaction.recv_channel, fcopy_poll_wrapper);
    }
}

#[no_mangle]
pub extern "C" fn fcopy_register_done_rust() -> () {
    pr_info!("Running Rust fcopy_register_done_rust");
    //below has the wrong "level" of logging, not sure what the equievelent level to pr_debug
    //would be on rust side, it should only display when CONFIG_DYNAMIC_DEBUG=y is set
    pr_info!("FCP: userspace daemon registered\n");
    unsafe {
        hv_poll_channel(fcopy_transaction.recv_channel, fcopy_poll_wrapper);
    }
}

#[no_mangle]
pub extern "C" fn fcopy_handle_handshake_rust() -> () {
    pr_info!("Running Rust fcopy_handle_handshake_rust");
}

#[no_mangle]
pub extern "C" fn fcopy_send_data_rust() -> () {
    pr_info!("Running Rust fcopy_send_data_rust");
}

#[no_mangle]
pub extern "C" fn fcopy_respond_to_host_rust() -> () {
    pr_info!("Running Rust fcopy_respond_to_host_rust");
}

#[no_mangle]
pub extern "C" fn hv_fcopy_onchannelcallback_rust() -> () {
    pr_info!("Running Rust hv_fcopy_onchannelcallback_rust");
}

#[no_mangle]
pub extern "C" fn fcopy_on_msg_rust() -> () {
    pr_info!("Running Rust fcopy_on_msg_rust");
}

#[no_mangle]
pub extern "C" fn fcopy_on_reset_rust() -> () {
    pr_info!("Running Rust fcopy_on_reset_rust");
}

#[no_mangle]
pub extern "C" fn hv_fcopy_init_rust(srv: *mut hv_util_service, hvt: *mut hvutil_transport, mut recv_buffer: *mut c_types::c_uchar) -> () {
    pr_info!("Running Rust hv_fcopy_init_rust");
    // unsafe {
    //     //recv_buffer = (*srv).recv_buffer;
    //     //fcopy_transaction.recv_channel = (*srv).channel as *mut vmbus_channel;
    //     //fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_INIT as c_types::c_int;
    // } 
}

#[no_mangle]
pub extern "C" fn hv_fcopy_cancel_work_rust() -> () {
    pr_info!("Running Rust hv_fcopy_cancel_work_rust");
    unsafe {
        //the function doesn't get run
        cancel_delayed_work_sync((&mut fcopy_timeout_work) as *mut delayed_work);
	    cancel_work_sync((&mut fcopy_send_work) as *mut work_struct);
    }
}

#[no_mangle]
pub extern "C" fn hv_fcopy_pre_suspend_rust() -> c_types::c_int {
    pr_info!("Running Rust hv_fcopy_pre_suspend_rust");
    unsafe {
        let channel: *mut vmbus_channel = fcopy_transaction.recv_channel;
    }
    0
}

#[no_mangle]
pub extern "C" fn hv_fcopy_pre_resume_rust() -> c_types::c_int {
    pr_info!("Running Rust hv_fcopy_pre_resume_rust");
    unsafe {
        let channel: *mut vmbus_channel = fcopy_transaction.recv_channel;
        //below references a function in rust/helpers.c which isn't inline like the original
        tasklet_enable(&mut((*channel).callback_event))
    }
    0
}

#[no_mangle]
pub extern "C" fn hv_fcopy_deinit_rust(hvt: *mut hvutil_transport) -> () {
    pr_info!("Running Rust hv_fcopy_deinit_rust");
    unsafe {
        fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_DYING as c_types::c_int;
        hv_fcopy_cancel_work();
        hvutil_transport_destroy(hvt);
    }
}