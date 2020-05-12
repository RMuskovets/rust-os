#[allow(dead_code)]

use crate::println;
use multiboot::{Multiboot, PAddr};

use core::{
    slice,
    mem
};

pub mod idt;
pub mod gdt;
pub mod interrupts;
pub mod memmap;

pub type MF<'a> = dyn Fn(PAddr, usize) -> Option<&'a [u8]>;

pub fn paddr_to_slice<'a>(p: multiboot::PAddr, sz: usize) -> Option<&'a [u8]> {
    unsafe {
        let ptr = mem::transmute(p);
        Some(slice::from_raw_parts(ptr, sz))
    }
}

pub fn initialize(mbaddr: usize) {
    let multiboot = unsafe { Multiboot::new(mbaddr as PAddr, paddr_to_slice) }
        .expect("Multiboot information is required");

    idt::init();
    gdt::init();
    interrupts::init();
    memmap::init(&multiboot);
    println!("[  OK  ] Hardware initialized successfully");
}