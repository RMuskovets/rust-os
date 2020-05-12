use multiboot::*;

use crate::{
    panic,
    println
};

pub fn init<'a, F: Fn(PAddr, usize) -> Option<&'a [u8]>>(multiboot: &'a Multiboot<'a, F>) {
    let regions = multiboot.memory_regions().expect("Memory regions are required!");
    for region in regions {
        println!("[ INFO ] Found memory region: start: {:#016x}, length: {:#016x}",
                 region.base_address(), region.length());
    }

    println!("[  OK  ] Memory mapping initialized successfully");
}