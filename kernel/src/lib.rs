#![no_std]

#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(unique)]
#![feature(slice_rotate)]

extern crate rlibc;
extern crate volatile;
extern crate spin;

mod lang;
mod vga;

use vga::Color;

/// Kernel main function
#[no_mangle]
pub extern fn kmain() -> ! {
    vga::WRITER.lock().fill_screen(Color::Black);
    vga::WRITER.lock().write_str("Flower kernel boot");
    vga::WRITER.lock().write_char('b');

    unsafe { asm!("hlt"); }
    loop {}
}
