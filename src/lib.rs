#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]

mod vga;

use core::panic::PanicInfo;
use core::fmt::Write;

static HELLO: &[u8] = b"hello, world!";

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rmain() { // short for "rust main"

    vga::WRITER.lock().puts("hello, world!").unwrap();

    loop {}
}