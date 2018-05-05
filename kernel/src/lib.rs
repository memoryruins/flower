#![no_std]

#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique, const_unique_new)]
#![feature(slice_rotate)]
#![feature(try_from)]
#![feature(nll)]
#![feature(range_contains, inclusive_range)]
#![feature(type_ascription)]
#![feature(ptr_internals, align_offset)]
#![feature(arbitrary_self_types)]
#![cfg_attr(test, feature(box_syntax))]

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate std;

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate x86_64;
extern crate array_init; // Used as a workaround until const-generics arrives
extern crate multiboot2;
extern crate bit_field;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

use core::mem;
use drivers::keyboard::{Keyboard, KeyEventType, Ps2Keyboard};
use drivers::keyboard::keymap;
use drivers::ps2;
use terminal::TerminalOutput;
use memory::bootstrap_heap;
use memory::physical_allocator::{PHYSICAL_ALLOCATOR, BLOCKS_IN_TREE, Block};

#[cfg(not(test))]
mod lang;
#[macro_use]
mod log;
#[macro_use]
mod util;
#[macro_use]
mod color;
mod io;
#[macro_use]
mod terminal;
mod memory;
mod drivers;

/// Kernel main function
#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    say_hello();

    // Parse and print mb info
    let mb_info = unsafe { multiboot2::load(multiboot_info_addr) };
    let memory_map = mb_info.memory_map_tag()
        .expect("Expected a multiboot2 memory map tag, but it is not present!");
    print_memory_info(memory_map);

    // Set up bootstrap heap
    let end_address = mb_info.end_address() as *const u8;
    let end_address = unsafe { end_address.offset(
        end_address.align_offset(mem::align_of::<[Block; BLOCKS_IN_TREE]>()) as isize
    )};
    let heap_start = end_address;
    unsafe { bootstrap_heap::BOOTSTRAP_HEAP.init_unchecked(heap_start as usize); }

    // Set up physical frame allocator
    PHYSICAL_ALLOCATOR.init(1, &[]); // TODO handle holes & # of GiB properly

    for _ in 0..4 {
        debug!("Allocated {:?}", PHYSICAL_ALLOCATOR.allocate(0).unwrap());
    }

    // Initialize the PS/2 controller and run the keyboard echo loop
    let mut controller = ps2::CONTROLLER.lock();
    match controller.initialize() {
        Ok(_) => info!("ps2c: init successful"),
        Err(error) => error!("ps2c: {:?}", error),
    }

    keyboard_echo_loop(&mut controller);

    halt()
}

/// Say hello to the user and print flower
fn say_hello() {
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

fn keyboard_echo_loop(controller: &mut ps2::Controller) {
    let keyboard_device = controller.device(ps2::DevicePort::Keyboard);
    let mut keyboard = Ps2Keyboard::new(keyboard_device);
    if let Ok(_) = keyboard.enable() {
        info!("kbd: successfully enabled");
        loop {
            if let Ok(Some(event)) = keyboard.read_event() {
                if event.event_type != KeyEventType::Break {
                    if event.keycode == keymap::codes::BACKSPACE {
                        // Ignore error
                        let _ = terminal::STDOUT.write().backspace();
                    } else if let Some(character) = event.char {
                        print!("{}", character)
                    }
                }
            }
        }
    } else {
        error!("kbd: enable unsuccessful");
    }
}

fn print_memory_info(memory_map: &multiboot2::MemoryMapTag) {
    debug!("mem: Usable memory areas: ");
    for area in memory_map.memory_areas() {
        debug!(" 0x{:x} to 0x{:x}",
                 area.start_address(), area.end_address());
    }
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
