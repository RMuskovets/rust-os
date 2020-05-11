use core::ops::Index;
use core::mem::size_of;
use core::marker::PhantomData;
use crate::panic;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelType {
    A_OUT {
        tabsize: u32,
        strsize: u32,
        addr: u32,
        reserved: u32
    },
    ELF {
        num: u32,
        size: u32,
        addr: u32,
        shndx: u32
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemRange {
    pub lower: u32,
    pub upper: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array<T: Sized> {
    pub count: u32,
    pub addr: u32,
    _phantom: PhantomData<T>
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBEInterface {
    pub segment: u16,
    pub offset: u16,
    pub len: u16
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Framebuffer {
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub typ: u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBE {
    pub ctrlinfo: u32,
    pub modeinfo: u32,
    pub mode: u16,
    pub interface: VBEInterface,
    pub framebuffer: Framebuffer
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Module {
    start: u32,
    end: u32,
    string: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Drive {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multiboot {
    pub flags: u32,
    pub memory: MemRange,
    pub bootdev: u32,
    pub cmdline: u32,
    pub mods: Array<Module>,
    pub ktype: KernelType,
    pub mmap: Array<Page>,
    pub drives: Array<Drive>,
    pub cfgtable: u32,
    pub loadername: u32,
    pub apmtable: u32,
    pub vbe: VBE
}

pub unsafe fn load(from: usize) -> Multiboot {
    *(from as *const Multiboot)
}

impl<T> Index<u32> for Array<T> where
    T: Copy {
    type Output = T;

    fn index(&self, idx: u32) -> &T {
        if idx >= self.count {
            panic!("Out of array bounds: count = {}, idx = {}", self.count, idx);
        }

        unsafe {
            let addr = self.addr + idx * size_of::<T>() as u32;
            &*((addr) as *const T)
        }
    }
}