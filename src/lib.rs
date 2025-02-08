pub mod event;
pub mod task;

extern crate alloc;

use event::Event;

fn default_event_handler(_event: &Event) -> bool {
    false
}

pub type EventHandler = fn(event: &Event) -> bool;

pub static mut EVENT_HANDLER: EventHandler = default_event_handler;

#[no_mangle]
pub extern "C" fn stride__allocate(size: usize) -> *mut core::ffi::c_void {
    let mut buffer = alloc::vec::Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    pointer
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn stride__deallocate(pointer: *mut core::ffi::c_void, capacity: usize) {
    unsafe {
        let _ = alloc::vec::Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn stride__event_handler(event: *const u8, event_len: usize) -> bool {
    let event_data = unsafe { core::slice::from_raw_parts(event, event_len) };
    let event: Event = serde_json::from_slice(event_data).unwrap();

    println!("event: {event:?}");

    EVENT_HANDLER(&event)
}
