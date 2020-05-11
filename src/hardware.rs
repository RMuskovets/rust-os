#[allow(dead_code)]

use crate::println;
use crate::multiboot::Multiboot;

pub mod idt;
pub mod gdt;
pub mod interrupts;
pub mod memmap;

pub fn initialize(multiboot: Multiboot) {
    idt::init();
    gdt::init();
    interrupts::init();
    memmap::init(multiboot);
    println!("[ INIT ] Hardware initialized successfully");
}