#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multiboot {
    pub flags: u32,
    pub memupper: u32,
    pub memlower: u32,
    pub bootdev: u32,
    pub cmdline: u32,
    pub modscnt: u32,
    pub modsaddr: u32,
    smth1: u32,
    smth2: u32,
    smth3: u32,
    smth4: u32,
    pub mmaplen: u32,
    pub mmapaddr: u32,
    pub driveslen: u32,
    pub drivesaddr: u32,
    pub cfgtable: u32,
    pub loadername: u32,
    pub apmtable: u32,
    pub vbectrlinfo: u32,
    pub vbemodeinfo: u32,
    pub vbemode: u16,
    pub vbeinterface_seg: u16,
    pub vbeinterface_off: u16,
    pub vbeinterface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8
}

pub unsafe fn load(from: usize) -> Multiboot {
    *(from as *const Multiboot)
}