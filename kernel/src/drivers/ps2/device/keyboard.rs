use drivers::ps2::{self, device::{self, Device, DevicePort}, Ps2Error};
use drivers::ps2::io::{self, command::device::keyboard::{Command, DataCommand}};
use spin::Mutex;

/// Represents a PS/2 scanset
#[allow(dead_code)] // Dead variants for completeness
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Scanset {
    One = 1,
    Two = 2,
    Three = 3,
}

bitflags! {
    pub struct LedFlags: u8 {
        /// If scroll lock is active
        const SCROLL_LOCK = 1 << 0;
        /// If number lock is active
        const NUMBER_LOCK = 1 << 1;
        /// If caps lock is active
        const CAPS_LOCK = 1 << 2;
    }
}

/// Represents a PS/2 scancode received from the device
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Scancode {
    pub code: u8,
    pub extended: bool,
    pub make: bool,
}

impl Scancode {
    pub fn new(scancode: u8, extended: bool, make: bool) -> Self {
        Scancode { code: scancode, extended, make }
    }
}

#[derive(Debug)]
pub struct Keyboard<'a> {
    internal: &'a Mutex<Option<DevicePort>>,
}

impl<'a> Keyboard<'a> {
    pub(super) const fn new(internal: &'a Mutex<Option<DevicePort>>) -> Self {
        Keyboard { internal }
    }

    pub fn set_scanset(&self, scanset: Scanset) -> Result<(), Ps2Error> {
        self.command_keyboard_data(DataCommand::SetGetScancode, scanset as u8)
    }

    pub fn get_scanset(&self) -> Result<Scanset, Ps2Error> {
        self.command_keyboard_data(DataCommand::SetGetScancode, 0)?;

        if let Some(response) = io::read_blocking(&io::DATA_PORT) {
            match response {
                1 => Ok(Scanset::One),
                2 => Ok(Scanset::Two),
                3 => Ok(Scanset::Three),
                v => Err(Ps2Error::UnexpectedResponse(v)),
            }
        } else {
            Err(Ps2Error::ExpectedResponse)
        }
    }

    pub fn set_leds(&self, leds: LedFlags) -> Result<(), Ps2Error> {
        self.command_keyboard_data(DataCommand::SetLeds, leds.bits())
    }

    fn command_keyboard(&self, cmd: Command) -> Result<(), Ps2Error> {
        if self.is_enabled() {
            let port = self.port().lock();
            device::command_raw(port.as_ref().ok_or(Ps2Error::DeviceUnavailable)?, cmd as u8)
        } else {
            Err(Ps2Error::DeviceDisabled)
        }
    }

    fn command_keyboard_data(&self, cmd: DataCommand, data: u8) -> Result<(), Ps2Error> {
        if self.is_enabled() {
            let port = self.port().lock();
            device::command_raw_data(port.as_ref().ok_or(Ps2Error::DeviceUnavailable)?, cmd as u8, data)
        } else {
            Err(Ps2Error::DeviceDisabled)
        }
    }
}

impl<'a> Device for Keyboard<'a> {
    fn port(&self) -> &Mutex<Option<DevicePort>> { self.internal }

    fn is_mouse(&self) -> bool { false }

    fn is_keyboard(&self) -> bool { true }
}

impl<'a> Keyboard<'a> {
    pub fn read_scancode(&self) -> Result<Option<Scancode>, Ps2Error> {
        if self.internal.lock().as_ref().ok_or(Ps2Error::DeviceUnavailable)?.can_read() {
            trace!("kbd: reading scancode");

            let mut make = true;
            let mut extended = false;

            // Get all scancode modifiers, and return when the actual scancode is received
            let scancode = loop {
                match ps2::io::read_blocking(&ps2::io::DATA_PORT) {
                    Some(0xE0...0xE1) => extended = true,
                    Some(0xF0) => make = false,
                    Some(data) => break Ok(data),
                    None => break Err(Ps2Error::ExpectedResponse),
                }
            }?;

            // If scancode is present, return it with modifiers
            Ok(if scancode != 0 {
                Some(Scancode::new(scancode, extended, make))
            } else {
                None
            })
        } else {
            Ok(None)
        }
    }
}
