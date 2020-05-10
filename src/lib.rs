#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "tmain"]

mod vga;

use core::panic::PanicInfo;
use core::fmt::Write;

static HELLO: &[u8] = b"hello, world!";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rmain() { // short for "rust main"

    println!("hello{}world{}", ", ", "!");

    tmain();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}