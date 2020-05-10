#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]

mod vga;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"hello, world!";

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rmain() { // short for "rust main"

    // let vga = 0xb8000 as *mut u8;
    // for (i, &b) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga.offset(i as isize * 2) = b;
    //         *vga.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    let mut wrt = vga::Writer {
        col: 0,
        color: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
        buf: unsafe { &mut *(0xb8000 as *mut vga::Buffer) }
    };

    wrt.puts("hello, world!");

    loop {}
}