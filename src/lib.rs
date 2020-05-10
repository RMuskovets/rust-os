// #![feature(lang_items)]
// #![no_std]

// #[no_mangle]
// pub extern fn rust_main() {

// }

// #[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
// #[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rmain() { // short for "rust main"
    loop {}
}