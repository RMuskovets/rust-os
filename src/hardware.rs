#[allow(dead_code)]

use crate::println;

pub mod idt;
pub mod gdt;
pub mod interrupts;

pub fn initialize() {
    idt::init();
    gdt::init();
    interrupts::init();
    println!("[ INIT ] Hardware initialized successfully");
}