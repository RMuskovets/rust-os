use core::ops::Index;
use core::mem::size_of;
use core::marker::PhantomData;
use crate::panic;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KernelType {
    pub field1: u32,
    pub field2: u32,
    pub field3: u32,
    pub field4: u32
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
    pub size: u32,
    pub addr: u64,
    pub len : u64,
    pub typ : u32
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayIter<T> {
    pub array: Array<T>,
    pub current_index: u32
}

pub unsafe fn load(from: usize) -> Multiboot {
    *(from as *const Multiboot)
}

impl<T> Index<u32> for Array<T> where
    T: Sized + Copy {
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

impl<T> Array<T> where T: Sized + Copy {

    pub fn get(&self, idx: u32) -> &T {
        &self[idx]
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn iter(&self) -> ArrayIter<T> {
        ArrayIter {
            array: *self,
            current_index: 0
        }
    }
}

// impl<T> Iterator for Array<T> where T: Sized + Copy {
//     type Item = T;
//
//     fn next(&self) -> Option<T> {
//
//     }
// }

impl<T> Iterator for ArrayIter<T> where T: Sized + Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current_index >= self.array.len() {
            None
        } else {
            let val = self.array[self.current_index];
            self.current_index += 1;
            Some(val)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PageType {
    AVAILABLE,
    ACPI_AVAILABLE,
    RESERVED_HIBERNATE,
    DEFECTIVE,
    RESERVED
}

impl Page {

    pub fn typ(&self) -> PageType {
        match self.typ {
            1 => PageType::AVAILABLE,
            3 => PageType::ACPI_AVAILABLE,
            4 => PageType::RESERVED_HIBERNATE,
            5 => PageType::DEFECTIVE,
            _ => PageType::RESERVED
        }
    }
}