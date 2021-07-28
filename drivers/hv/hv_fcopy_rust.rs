#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::bindings::*;
use kernel::c_types;
use core::mem;

extern "C" {
    fn hv_fcopy_cancel_work() -> ();
    fn hvutil_transport_destroy(hvt: *mut hvutil_transport) -> ();
    fn tasklet_enable(t: *mut tasklet_struct) -> ();
    fn tasklet_schedule(t: *mut tasklet_struct) -> ();
    fn tasklet_disable(t: *mut tasklet_struct) -> ();
    fn fcopy_respond_to_host(error: c_types::c_int) -> ();
    fn fcopy_poll_wrapper(channel: *mut c_types::c_void);
    fn hv_poll_channel(channel: *mut vmbus_channel, x: unsafe extern "C" fn(channel: *mut c_types::c_void));
    fn vmbus_sendpacket(channel: *mut vmbus_channel, buffer: *mut c_types::c_uchar, bufferLen: c_types::c_uint, requestid: c_types::c_ulonglong, packet_type: vmbus_packet_type, flags: c_types::c_uint) -> c_types::c_int;
    fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool;
    fn hvutil_transport_init(name: *mut c_types::c_char, cn_idx: c_types::c_uint, cn_val: c_types::c_uint, x: unsafe extern "C" fn(msg: *mut c_types::c_void, len: c_types::c_int) -> c_types::c_int, on_reset: unsafe extern "C" fn()) -> *mut hvutil_transport;
    fn fcopy_register_done() -> ();
    fn fcopy_on_msg(msg: *mut c_types::c_void, len: c_types::c_int) -> c_types::c_int;
    fn fcopy_on_reset()->();
    fn fcopy_handle_handshake(version: c_types::c_uint) -> c_types::c_int;
    fn kzalloc_fn(size: c_types::c_size_t, flags: gfp_t) -> *mut c_types::c_void;
    fn kfree(p: *mut c_types::c_void) -> ();
    //fn vmbus_recvpacket(channel: *mut vmbus_channel, buffer: *mut c_types::c_void, bufferlen: u32, buffer_actual_len: *mut u32, requestid: *mut u64) -> c_types::c_int;
    fn schedule_work_rust(work: *mut work_struct)->bool;
    fn schedule_delayed_work_rust(dwork: *mut delayed_work, delay: c_types::c_ulong)->bool;
    static mut fcopy_timeout_work: delayed_work;
    static mut fcopy_send_work: work_struct;
    static mut fcopy_transaction: fcopy_transaction_struct;
    static mut dm_reg_value: c_types::c_int;
    static mut hvt: *mut hvutil_transport;
    static fcopy_devname: [c_types::c_char;14];
    static fcopy_versions: [i32;1];
    static fw_versions: [i32;1];
}
const __LOG_PREFIX: &[u8] = b"rust_hv\0";

#[no_mangle]
pub extern "C" fn fcopy_poll_wrapper_rust(channel: *mut c_types::c_void) -> () {
    unsafe {
        fcopy_transaction.state = hvutil_device_state_HVUTIL_READY as c_types::c_int;
        tasklet_schedule(&mut((*(channel as *mut vmbus_channel)).callback_event))
    }
}

#[no_mangle]
pub extern "C" fn fcopy_timeout_func_rust() -> () {
    unsafe {
        fcopy_respond_to_host(HV_E_FAIL as c_types::c_int);
        hv_poll_channel(fcopy_transaction.recv_channel, fcopy_poll_wrapper);
    }
}

#[no_mangle]
pub extern "C" fn fcopy_register_done_rust() -> () {
    pr_info!("FCP: userspace daemon registered\n");
    unsafe {
        hv_poll_channel(fcopy_transaction.recv_channel, fcopy_poll_wrapper);
    }
}

#[no_mangle]
pub extern "C" fn fcopy_handle_handshake_rust(version: c_types::c_uint) -> c_types::c_int {
    let mut our_ver: c_types::c_uint = FCOPY_CURRENT_VERSION;

    match version {
        FCOPY_VERSION_0 => unsafe { dm_reg_value = version as i32 },
        FCOPY_VERSION_1 => {
            unsafe { 
                let result: c_types::c_int = hvutil_transport_send(hvt, &our_ver as *const _ as *const _ as *mut c_types::c_void, mem::size_of::<u32>() as i32, Some(fcopy_register_done));  
                if result != 0 {
                    return -(EFAULT as c_types::c_int);
                }
                dm_reg_value = version as i32; 
            }
        }
        _ => return -(EINVAL as c_types::c_int),
    }
    pr_info!("FCP: userspace daemon ver. {} connected\n", version);
    0
}

#[no_mangle]
pub extern "C" fn fcopy_send_data_rust(dummy: *mut work_struct) -> () {
    unsafe {
        let mut smsg_out: *mut hv_start_fcopy = core::ptr::null::<hv_start_fcopy>() as *mut hv_start_fcopy;
        let mut operation: c_types::c_uint = (*fcopy_transaction.fcopy_msg).operation;
        let mut smsg_in: *mut hv_start_fcopy;
        let mut out_src: *mut c_types::c_void;
        let mut rc: c_types::c_int;
        let mut out_len: c_types::c_int;

        match operation {
            hv_fcopy_op_START_FILE_COPY => {
                out_len = mem::size_of::<hv_start_fcopy>() as i32;
                smsg_out = kzalloc_fn(mem::size_of::<hv_start_fcopy>(), BINDINGS_GFP_KERNEL) as *mut hv_start_fcopy;
                if smsg_out.is_null() {
                    return;
                }
                (*smsg_out).hdr.operation = operation;
                smsg_in = fcopy_transaction.fcopy_msg as *mut hv_start_fcopy;

                utf16s_to_utf8s((*smsg_in).file_name.as_ptr(), 260, utf16_endian_UTF16_LITTLE_ENDIAN, 
                    (*smsg_out).file_name.as_ptr() as *mut u8, 260-1);
                utf16s_to_utf8s((*smsg_in).path_name.as_ptr(), 260, utf16_endian_UTF16_LITTLE_ENDIAN, 
                    (*smsg_out).path_name.as_ptr() as *mut u8, 260-1);

                (*smsg_out).copy_flags = (*smsg_in).copy_flags;
                (*smsg_out).file_size = (*smsg_in).file_size;

                out_src = smsg_out as *mut c_types::c_void;

            }
            hv_fcopy_op_WRITE_TO_FILE => {
                out_src = fcopy_transaction.fcopy_msg as *mut c_types::c_void;
                out_len = mem::size_of::<hv_do_fcopy>() as i32;
            }
            _ => {
                out_src = fcopy_transaction.fcopy_msg as *mut c_types::c_void;
                out_len = fcopy_transaction.recv_len;
            }
        }

        fcopy_transaction.state = hvutil_device_state_HVUTIL_USERSPACE_REQ as i32;

        rc = hvutil_transport_send(hvt, out_src, out_len, None);

        if rc != 0 {
            pr_info!("FCP: failed to communicate to the daemon: {}\n", rc);
            if cancel_delayed_work_sync(&mut fcopy_timeout_work) {
                fcopy_respond_to_host(HV_E_FAIL as i32);
			    fcopy_transaction.state = hvutil_device_state_HVUTIL_READY as i32;
            }
        }
        kfree(smsg_out as *mut c_types::c_void);
    }
}

#[no_mangle]
pub extern "C" fn fcopy_respond_to_host_rust(error: c_types::c_int) -> () {
    unsafe{
        let mut icmsghdr: *mut icmsg_hdr;
        let mut buf_len: c_types::c_uint;
        let mut channel: *mut vmbus_channel;
        let mut req_id: c_types::c_ulonglong;


        buf_len = fcopy_transaction.recv_len as u32;
        channel = fcopy_transaction.recv_channel;
        req_id = fcopy_transaction.recv_req_id;

        icmsghdr = recv_buffer.offset(mem::size_of::<vmbuspipe_hdr>() as isize) as *mut icmsg_hdr;

        // if ((*channel).onchannel_callback.is_null()) {
        //     return;
        // }//

        (*icmsghdr).status = error as u32;
        (*icmsghdr).icflags = (ICMSGHDRFLAG_TRANSACTION | ICMSGHDRFLAG_RESPONSE) as u8;
    
        vmbus_sendpacket(channel, recv_buffer, buf_len, req_id,
            vmbus_packet_type_VM_PKT_DATA_INBAND, 0);
    }
}

#[no_mangle]
pub extern "C" fn hv_fcopy_onchannelcallback_rust(context: *mut c_types::c_void) -> () {
    pr_info!("enterings onchannelcallback");
    unsafe {
        let ICMSG_HDR = (mem::size_of::<vmbuspipe_hdr>() + mem::size_of::<icmsg_hdr>()) as u32;
        let mut channel: *mut vmbus_channel = context as *mut vmbus_channel;
        let mut recvlen: u32 = 0;
        let mut requestid: u64 = 0;
        let mut fcopy_msg: *mut hv_fcopy_hdr;
        let mut icmsghdr: *mut icmsg_hdr;
        let mut fcopy_srv_version: i32 = 0;
        
        if fcopy_transaction.state > hvutil_device_state_HVUTIL_READY as i32 {
            return;
        }
    
        if vmbus_recvpacket(channel, recv_buffer as *mut c_types::c_void, 1<<13, &mut recvlen, &mut requestid)!=0 {
            pr_info!("Fcopy request received. Could not read into recv buf\n");
            return;
        }

        if recvlen == 0 {
            pr_info!("exiting here");
            return;
        }
        if recvlen < ICMSG_HDR {
            pr_info!("Fcopy request received. Packet length too small: {}\n", recvlen);
            return;
        }

        icmsghdr = recv_buffer.offset(mem::size_of::<vmbuspipe_hdr>() as isize) as *mut icmsg_hdr;

        if (*icmsghdr).icmsgtype == ICMSGTYPE_NEGOTIATE as u16 {
            if vmbus_prep_negotiate_resp(icmsghdr,
                    recv_buffer, recvlen,
                    &fw_versions as *const i32, FCOPY_VER_COUNT as i32,
                    &fcopy_versions as *const i32, FW_VER_COUNT as i32,
                    &mut 0, &mut fcopy_srv_version) {
                pr_info!("FCopy IC version {}.{}\n",
                fcopy_srv_version >> 16,
                fcopy_srv_version & 0xFFFF);
            }
        }
        else if (*icmsghdr).icmsgtype == ICMSGTYPE_FCOPY as u16 {
            if recvlen < ICMSG_HDR + mem::size_of::<hv_fcopy_hdr>() as u32 {

                pr_info!("Invalid Fcopy hdr. Packet length too small: {}\n", recvlen);
                return;
            }

            fcopy_msg = recv_buffer.offset(ICMSG_HDR as isize) as *mut hv_fcopy_hdr;

            fcopy_transaction.recv_len = recvlen as i32;
            fcopy_transaction.recv_req_id = requestid;
            fcopy_transaction.fcopy_msg = fcopy_msg;

            if fcopy_transaction.state < hvutil_device_state_HVUTIL_READY as i32 {
                fcopy_respond_to_host(HV_E_FAIL as i32);
                return;
            }

            fcopy_transaction.state = hvutil_device_state_HVUTIL_HOSTMSG_RECEIVED as i32;

            schedule_work_rust(&mut fcopy_send_work);
            schedule_delayed_work_rust(&mut fcopy_timeout_work,
                3000);
            return;
        } else {
            pr_info!("Fcopy request received. Invalid msg type: {}\n",
				   (*icmsghdr).icmsgtype);
		    return;
        }
        (*icmsghdr).icflags = (ICMSGHDRFLAG_TRANSACTION | ICMSGHDRFLAG_RESPONSE) as u8;
	    vmbus_sendpacket(channel, recv_buffer, recvlen, requestid,
			vmbus_packet_type_VM_PKT_DATA_INBAND, 0);
    }
}

#[no_mangle]
pub extern "C" fn fcopy_on_msg_rust(msg: *mut c_types::c_void, len: c_types::c_int) -> c_types::c_int {
    unsafe {
        let mut val: *mut c_types::c_int = msg as *mut c_types::c_int;
        if len != mem::size_of::<c_types::c_int>() as i32 {
            return -(EINVAL as c_types::c_int)
        }
        if fcopy_transaction.state == hvutil_device_state_HVUTIL_DEVICE_INIT as i32 {
            return fcopy_handle_handshake(*val as u32);
        }
        if fcopy_transaction.state != hvutil_device_state_HVUTIL_USERSPACE_REQ as i32 {
            return -(EINVAL as c_types::c_int)
        }

        if cancel_delayed_work_sync(&mut fcopy_timeout_work) {
            fcopy_transaction.state = hvutil_device_state_HVUTIL_USERSPACE_RECV as i32;
            fcopy_respond_to_host(*val);
            hv_poll_channel(fcopy_transaction.recv_channel, fcopy_poll_wrapper);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn fcopy_on_reset_rust() -> () {
    unsafe {
        fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_INIT as i32;

        if cancel_delayed_work_sync(&mut fcopy_timeout_work) {
            fcopy_respond_to_host(HV_E_FAIL as i32);
        }
    }
}

#[no_mangle]
pub extern "C" fn hv_fcopy_init_rust(srv: *mut hv_util_service) -> c_types::c_int {
    unsafe {
        recv_buffer = (*srv).recv_buffer;
        fcopy_transaction.recv_channel = (*srv).channel as *mut vmbus_channel;

        fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_INIT as c_types::c_int;

        hvt = hvutil_transport_init(fcopy_devname.as_ptr() as *mut i8, 0, 0, fcopy_on_msg, fcopy_on_reset);

        if hvt.is_null() { 
            return -(EFAULT as c_types::c_int)
        }
    } 
    0
}

#[no_mangle]
pub extern "C" fn hv_fcopy_cancel_work_rust() -> () {
    unsafe {
        cancel_delayed_work_sync((&mut fcopy_timeout_work) as *mut delayed_work);
	    cancel_work_sync((&mut fcopy_send_work) as *mut work_struct);
    }
}

#[no_mangle]
pub extern "C" fn hv_fcopy_pre_suspend_rust() -> c_types::c_int {
    unsafe {
        let mut channel: *mut vmbus_channel = fcopy_transaction.recv_channel;
        let mut fcopy_msg: *mut hv_fcopy_hdr;

        fcopy_msg = kzalloc_fn(mem::size_of::<hv_fcopy_hdr>(), BINDINGS_GFP_KERNEL) as *mut hv_fcopy_hdr;

        if fcopy_msg.is_null() {
            return -(ENOMEM as c_types::c_int)
        }

        tasklet_disable(&mut((*channel).callback_event));

        (*fcopy_msg).operation = hv_fcopy_op_CANCEL_FCOPY;

        hv_fcopy_cancel_work();

        hvutil_transport_send(hvt, fcopy_msg as *mut c_types::c_void, mem::size_of::<hv_fcopy_hdr>() as i32, None);

        kfree(fcopy_msg as *mut c_types::c_void);

        fcopy_transaction.state = hvutil_device_state_HVUTIL_READY as i32;
    }
    0
}

#[no_mangle]
pub extern "C" fn hv_fcopy_pre_resume_rust() -> c_types::c_int {
    unsafe {
        let mut channel: *mut vmbus_channel = fcopy_transaction.recv_channel;
        tasklet_enable(&mut((*channel).callback_event))
    }
    0
}

#[no_mangle]
pub extern "C" fn hv_fcopy_deinit_rust() -> () {
    unsafe {
        fcopy_transaction.state = hvutil_device_state_HVUTIL_DEVICE_DYING as c_types::c_int;
        hv_fcopy_cancel_work();
        hvutil_transport_destroy(hvt);
    }
}
