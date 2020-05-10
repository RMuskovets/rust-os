#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"hello, world!";

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rmain() { // short for "rust main"

    let vga = 0xb8000 as *mut u8;
    for (i, &b) in HELLO.iter().enumerate() {
        unsafe {
            *vga.offset(i as isize * 2) = b;
            *vga.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}