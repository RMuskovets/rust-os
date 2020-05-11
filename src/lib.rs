#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "tmain"]
#![feature(abi_x86_interrupt)]

mod vga;
mod hardware;
mod multiboot;

use core::panic::PanicInfo;

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[ ERR  ] PANIC: {}", info);
    hlt_loop();
}

#[no_mangle]
pub extern "C" fn rmain(mbaddr: usize) { // short for "rust main"

    // let multiboot_info = unsafe { multiboot2::load(multiboot) };

    let multiboot_info = unsafe { multiboot::load(mbaddr) };

    vga::WRITER.lock().clear();

    hardware::initialize(multiboot_info);

    println!("[ INIT ] Everything initialized successfully");

    println!("[ INFO ] VESA mode {:#02x} = {}x{} {}bpp",
        multiboot_info.vbe.mode,
        multiboot_info.vbe.framebuffer.width, multiboot_info.vbe.framebuffer.height,
        multiboot_info.vbe.framebuffer.bpp);
    println!("[ INFO ] Framebuffer located at {:#016x}",
        multiboot_info.vbe.framebuffer.addr);



    #[cfg(test)]
    tmain();

    hlt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("[ TEST ] Running {} tests", tests.len());
    for test in tests {
        test();
    }
}