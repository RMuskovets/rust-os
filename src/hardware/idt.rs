#[allow(dead_code)]

use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame
};
use crate::{
    print,
    println,
    hardware
};
use lazy_static::lazy_static;
use crate::hardware::interrupts;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.breakpoint.set_handler_fn(on_breakpoint)
                .set_stack_index(hardware::gdt::DOUBLE_FAULT_IST);
        }
        idt.double_fault.set_handler_fn(on_double_fault);
        idt[interrupts::Index::Timer.as_usize()].set_handler_fn(on_timer_tick);
        idt[interrupts::Index::Keyboard.as_usize()].set_handler_fn(on_keyboard);
        idt
    };
}

pub fn init() {
    IDT.load();
    println!("[ INIT ] IDT loaded successfully");
}

extern "x86-interrupt" fn on_breakpoint(frame: &mut InterruptStackFrame) {
    println!("[ INFO ] Breakpoint: {:#?}", frame);
}

extern "x86-interrupt" fn on_double_fault(frame: &mut InterruptStackFrame, errorcode: u64) -> ! {
    panic!("[ ERR  ] Double Fault (code {}): {:#?}", errorcode, frame);
}

extern "x86-interrupt" fn on_timer_tick(frame: &mut InterruptStackFrame) {

    unsafe {
        interrupts::PICS.lock()
            .notify_end_of_interrupt(interrupts::Index::Timer.as_u8());
    }
}

extern "x86-interrupt" fn on_keyboard(frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let code: u8 = unsafe { port.read() };
    
    if let Ok(Some(key_event)) = keyboard.add_byte(code) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(c) => print!("{}", c),
                DecodedKey::RawKey(key) => print!("{:?}", key)
            }
        }
    }

    unsafe {
        interrupts::PICS.lock()
            .notify_end_of_interrupt(interrupts::Index::Keyboard.as_u8());
    }
}