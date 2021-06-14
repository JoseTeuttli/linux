
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
pub extern "C" fn rust_hv_get_next_write_location(ring_info: *mut hv_ring_buffer_info) -> c_types::c_uint {
    print_hello();
    unsafe {
    pr_info!("Trying to read from passed data structure got int: {}", (*(*ring_info).ring_buffer).write_index);
    (*(*ring_info).ring_buffer).write_index
    }
}
#[no_mangle]
pub extern "C" fn hv_get_next_write_location(ring_info: *mut hv_ring_buffer_info) -> c_types::c_uint {
    pr_info!("Running Rust hv_get_next_write_location");
    unsafe {
    (*(*ring_info).ring_buffer).write_index
    }
}

#[no_mangle]
pub extern "C" fn hv_set_next_write_location(ring_info: *mut hv_ring_buffer_info, next_write_location: c_types::c_uint) -> () {
    pr_info!("Running Rust hv_set_next_write_location");
    unsafe {
    (*(*ring_info).ring_buffer).write_index = next_write_location;
    }
    ()
}

#[no_mangle]
pub extern "C" fn hv_set_next_read_location(ring_info: *mut hv_ring_buffer_info, next_read_location: c_types::c_uint) -> () {
    pr_info!("Running Rust hv_set_next_read_location");
    unsafe {
        (*(*ring_info).ring_buffer).read_index = next_read_location;
        (*ring_info).priv_read_index = next_read_location;
    }
}

//function below currently not working due to not knowing how to best either use the macro on the c side or copy it over to the rust side WIP
// #[no_mangle]
// pub extern "C" fn hv_signal_on_write(old_write: c_types::c_uint, channel: *mut vmbus_channel) -> () {
//     pr_info!("Running Rust hv_signal_on_write");
//     let rbi: *mut hv_ring_buffer_info = (&channel).outbound;
//     virt_mb();
//     if READ_ONCE((*(*rbi).ring_buffer).interrupt_mask) {
//         return;
//     }

//     virt_mb();

//     if (old_write == READ_ONCE((*(*rbi).ring_buffer).read_index)) {
//         (*channel).intr_out_empty = (*channel).intr_out_empty+1;
//         vmbus_setevent(channel);
//     }
// }

// static void hv_signal_on_write(u32 old_write, struct vmbus_channel *channel)
// {
// 	struct hv_ring_buffer_info *rbi = &channel->outbound;

// 	virt_mb();
// 	if (READ_ONCE(rbi->ring_buffer->interrupt_mask))
// 		return;

// 	/* check interrupt_mask before read_index */
// 	virt_rmb();
// 	/*
// 	 * This is the only case we need to signal when the
// 	 * ring transitions from being empty to non-empty.
// 	 */
// 	if (old_write == READ_ONCE(rbi->ring_buffer->read_index)) {
// 		++channel->intr_out_empty;
// 		vmbus_setevent(channel);
// 	}
// }