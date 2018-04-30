#![no_std]

#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique, const_unique_new)]
#![feature(slice_rotate)]
#![feature(try_from)]
#![feature(nll)]
#![feature(try_trait)]
#![feature(type_ascription)]
#![feature(ptr_internals)]

extern crate array_init;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

mod lang;
#[macro_use]
mod log;
#[macro_use]
mod util;
#[macro_use]
mod color;
#[macro_use]
mod terminal;
mod io;
mod drivers;

use drivers::ps2;
//use drivers::keyboard::{Keyboard, KeyEventType, Ps2Keyboard};
use terminal::TerminalOutput;

/// Kernel main function
#[no_mangle]
pub extern fn kmain() -> ! {
    terminal::STDOUT.write().clear().expect("Screen clear failed");

    print_flower().expect("Flower print failed");

    terminal::STDOUT.write().set_color(color!(Green on Black))
        .expect("Color should be supported");

    // Print boot message
    println!("Flower kernel boot!");
    println!("-------------------\n");

    // Reset colors
    terminal::STDOUT.write().set_color(color!(White on Black))
        .expect("Color should be supported");

    let mut controller = ps2::Controller::new();
    match controller.setup() {
        Ok(_) => info!("ps2c: successful setup"),
        Err(error) => error!("ps2c: threw error: {:?}", error),
    }

    if let Ok(keyboard) = controller.keyboard() {
//        let mut keyboard = Ps2Keyboard::new(keyboard);
//
//        if let Ok(_) = keyboard.enable() {
//            println!("kbd: successfully enabled");
//            loop {
//                if let Ok(Some(event)) = keyboard.read_event() {
//                    if event.event_type != KeyEventType::Break {
//                        if let Some(char) = event.char {
//                            print!("{}", char);
//                        }
//                    }
//                }
//            }
//        } else {
//            println!("kbd: enable unsuccessful");
//        }
    } else {
        warn!("kbd: not available");
    }

    halt()
}

fn print_flower() -> Result<(), terminal::TerminalOutputError<()>> {
    const FLOWER: &'static str = include_str!("resources/art/flower.txt");
    const FLOWER_STEM: &'static str = include_str!("resources/art/flower_stem.txt");

    let mut stdout = terminal::STDOUT.write();
    let old = stdout.cursor_pos();

    stdout.write_string_colored(FLOWER, color!(LightBlue on Black))?;
    stdout.write_string_colored(FLOWER_STEM, color!(Green on Black))?;
    stdout.set_cursor_pos(old)
}

fn halt() -> ! {
    unsafe {
        // Disable interrupts
        asm!("cli");

        // Halt forever...
        loop {
            asm!("hlt");
        }
    }
}
