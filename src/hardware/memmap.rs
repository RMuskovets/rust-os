use crate::multiboot::{
    Multiboot,
    PageType
};

use crate::{
    panic,
    println
};

pub fn init(multiboot: Multiboot) {
    if (multiboot.flags >> 6) & 1 != 1 {
        panic!("[  ERR ] Memory map not provided by GRUB (bit 6 in {:#018b} = {})!",
               multiboot.flags, multiboot.flags & (1 << 6));
    }

    let pages = multiboot.mmap;
    let mut avail: u64 = 0;

    println!("[ INFO ] Pages count: {}", pages.count);

    for page in pages.iter() {
        // match page.typ() {
        //     PageType::AVAILABLE => {
        //         println!("[ INFO ] Found page: start: {:#016x}, length: {:#016x}",
        //                  page.addr, page.len);
        //     },
        //     _ => {}
        // }
        println!("[ INFO ] Found page: start: {:#016x}, size: {:#016x}, status: {}", page.addr, page.len, page.typ);
        if page.typ() == PageType::AVAILABLE {
            avail += 1;
        }
    }

    println!("[ INFO ] Available pages: {}", avail);
}