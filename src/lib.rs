#![no_std]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

use cty::c_void;
use nspire::prelude::*;

#[cfg(not(feature = "disable-eh-personality"))]
#[lang = "eh_personality"]
extern fn eh_personality() {}


#[cfg(not(feature = "disable-oom-handler"))]
#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! {
	unsafe { ndless_sys::abort(); }
}


#[cfg(not(feature = "disable-panic-handler"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	let msg = match info.message() {
		Some(err) => format!("An error occured: {}", err),
		None => "An error occured!".to_string()
	};
	let location = match info.location() {
		Some(loc) => format!("In file {} at line {} column {}", loc.file(), loc.line(), loc.column()),
		None => "".to_string(),
	};
	nspire::msg::msg("Error", format!("{}\n{}", msg, location));
	unsafe { ndless_sys::abort() }
}

/// This allows for dynamic allocation, which calls the C functions `calloc` and `free`.
struct CAllocator;

unsafe impl GlobalAlloc for CAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		ndless_sys::calloc(1, layout.size()) as *mut u8
	}
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		ndless_sys::free(ptr as *mut c_void)
	}
}

#[cfg(not(feature = "disable-allocator"))]
#[global_allocator]
static A: CAllocator = CAllocator;

#[cfg(not(feature = "disable_ctype_ptr"))]
#[no_mangle]
pub static __ctype_ptr__: [u8; 128 + 256] = [0; 128 + 256];
