pub mod event;
pub mod task;

extern crate alloc;

use event::{EmitEvent, Event};

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

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stride__deallocate(pointer: *mut core::ffi::c_void, capacity: usize) {
    unsafe {
        let _ = alloc::vec::Vec::from_raw_parts(pointer, 0, capacity);
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stride__event_handler(event: *const u8, event_len: usize) -> bool {
    let event_data = unsafe { core::slice::from_raw_parts(event, event_len) };
    let event: Event = serde_json::from_slice(event_data).unwrap();

    EVENT_HANDLER(&event)
}

extern "C" {
    pub fn stride__emit(event: *const u8, event_len: usize);
}

pub trait Plugin {
    fn init() -> Self;
    fn event(&mut self, event: &Event) -> bool;
}

#[macro_export]
macro_rules! plugin {
    ($plugin:ty) => {
        #[allow(non_snake_case)]
        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn stride__init() {
            pub static PLUGIN_INSTANCE: std::sync::LazyLock<std::sync::Mutex<$plugin>> =
                std::sync::LazyLock::new(|| std::sync::Mutex::new(<$plugin>::init()));

            unsafe {
                $crate::EVENT_HANDLER = |event| {
                    PLUGIN_INSTANCE
                        .lock()
                        .expect("couldn't lock plugin instance")
                        .event(event)
                };
            }
        }
    };
}

pub fn emit(event: &EmitEvent) {
    let event = serde_json::to_string(event).expect("should not fail");

    unsafe {
        stride__emit(event.as_bytes().as_ptr(), event.len());
    }
}
