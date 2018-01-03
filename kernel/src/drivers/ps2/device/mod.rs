// TODO examples out of date

use core::convert::TryFrom;
use super::Ps2Error;
use super::io::{self, DEVICE_DATA_PORT, STATUS_COMMAND_PORT};
use super::io::command::{controller::ControllerCommand, device::DeviceCommand};

/// Represents the state of a device.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceState {
    /// The device has been detected but is not enabled
    Available, // TODO are these right even tho
    /// The device has been enabled
    Enabled,
}

/// Represents the type of a device
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceType {
    TranslatedAtKeyboard,
    Mouse,
    MouseWithScrollWheel,
    FiveButtonMouse,
    Mf2Keyboard,
    TranslatedMf2Keyboard,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UnknownDevice {
    pub identifier: u8
}

impl TryFrom<u8> for DeviceType {
    type Error = UnknownDevice;

    fn try_from(value: u8) -> Result<Self, UnknownDevice> {
        use self::DeviceType::*;

        match value {
            0x00 => Ok(Mouse),
            0x03 => Ok(MouseWithScrollWheel),
            0x04 => Ok(FiveButtonMouse),
            0xAB | 0x83 => Ok(Mf2Keyboard),
            // According to Osdev wiki, a translated MF2 keyboard can also be 0xAB, but here it is
            // given higher priority to MF2 keyboards proper
            0x41 | 0xC1 => Ok(TranslatedMf2Keyboard),
            identifier => Err(UnknownDevice { identifier }),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DevicePort {
    One,
    Two
}

/// A PS2 device
// TODO mut for device or not? Cpu io mut or not?
pub trait Device {
    fn port(&self) -> DevicePort;

    /// Enables this device
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let controller = ps2::CONTROLLER.lock();
    /// let device = controller.device(DevicePort::Keyboard);
    /// match device.enable() {
    ///     Ok(()) => println!("Device enabled"),
    ///     Err(err) => println!("Error enabling device: {:?}", err)
    /// }
    ///
    /// ```
    fn enable(&self) {
        let cmd = if self.port() == DevicePort::One {
            ControllerCommand::EnablePort1
        } else {
            ControllerCommand::EnablePort2
        };
        io::write(&STATUS_COMMAND_PORT, cmd as u8);
    }

    /// Disables this device
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let controller = ps2::CONTROLLER.lock();
    /// let device = controller.device(DevicePort::Keyboard);
    /// match device.disable() {
    ///     Ok(()) => println!("Device disabled"),
    ///     Err(err) => println!("Error disabling device: {:?}", err)
    /// }
    /// ```
    // TODO must be public?
    fn disable(&self) {
        let cmd = if self.port() == DevicePort::One {
            ControllerCommand::DisablePort1
        } else {
            ControllerCommand::DisablePort2
        };
        io::write(&STATUS_COMMAND_PORT, cmd as u8);
    }

    /// Enables scanning for this device. This will signal the device to begin sending data again,
    /// like scancodes from keyboards and updates from mice.
    fn enable_scanning(&self) -> Result<(), Ps2Error> {
        command(self, DeviceCommand::EnableScanning as u8)
    }

    /// Disables scanning for this device. This will stop the device from sending data, like
    /// scancodes from keyboards and updates from mice.
    fn disable_scanning(&self) -> Result<(), Ps2Error> {
        command(self, DeviceCommand::DisableScanning as u8)
    }

    /// Sets the parameters of this device to default parameters
    fn set_defaults(&self) -> Result<(), Ps2Error> {
        command(self, DeviceCommand::SetDefaults as u8)
    }

    /// Resets this device
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let controller = ps2::CONTROLLER.lock();
    /// let device = controller.device(DevicePort::Keyboard);
    /// match device.reset() {
    ///     Ok(()) => println!("Device reset"),
    ///     Err(err) => println!("Error resetting device: {:?}", err)
    /// }
    /// ```
    fn reset(&self) -> Result<(), Ps2Error> {
        command(self, DeviceCommand::Reset as u8)
    }

    fn identify(&self) -> Result<DeviceType, Ps2Error> {
        // TODO
        unimplemented!()
    }

    // TODO caching with interrupts?
    fn state(&self) -> Result<DeviceState, Ps2Error> {
        // TODO
        unimplemented!()
    }
}

/// Sends a raw command code to this device
fn command<D: Device + ?Sized>(device: &D, cmd: u8) -> Result<(), Ps2Error> {
    if device.state()? != DeviceState::Enabled {
        return Err(Ps2Error::DeviceDisabled)
    }

    // If second PS2 port, send context switch command
    if device.port() == DevicePort::Two {
        io::write(&STATUS_COMMAND_PORT, ControllerCommand::WriteInputPort2 as u8);
    }

    // TODO this whole pattern can go into a function ew
    for _ in 0..io::RETRIES {
        io::write(&io::DEVICE_DATA_PORT, cmd);
        match io::read(&DEVICE_DATA_PORT) {
            Some(io::RESEND) => continue,
            Some(io::ACK) => return Ok(()), // TODO check dis
            Some(unknown) => return Err(Ps2Error::UnexpectedResponse(unknown)),
            None => return Err(Ps2Error::ExpectedResponse),
        }
    }

    Ok(())
}

/// Sends a raw command code to this device and returns
// TODO no?
// TODO needs mut?
fn command_ret<D: Device + ?Sized>(device: &D, cmd: u8) -> Result<u8, Ps2Error> {
    if device.state()? != DeviceState::Enabled {
        return Err(Ps2Error::DeviceDisabled)
    }

    if device.port() == DevicePort::Two {
        io::write(&STATUS_COMMAND_PORT, ControllerCommand::WriteInputPort2 as u8);
    }

    for _ in 0..io::RETRIES {
        io::write(&io::DEVICE_DATA_PORT, cmd);
        match io::read(&DEVICE_DATA_PORT) {
            Some(io::RESEND) => continue,
            Some(result) => return Ok(result),
            None => return Err(Ps2Error::ExpectedResponse)
        }
    }

    Err(Ps2Error::ExpectedResponse)
}

/// Sends a raw command to this PS2 device with data
// TODO needs mut?
fn command_data<D: Device + ?Sized>(device: &D, cmd: u8, data: u8) -> Result<(), Ps2Error> {
    if device.state()? != DeviceState::Enabled {
        return Err(Ps2Error::DeviceDisabled)
    }

    // If second PS2 port, send context switch command
    if device.port() == DevicePort::Two {
        io::write(&STATUS_COMMAND_PORT, ControllerCommand::WriteInputPort2 as u8);
    }

    let mut ok = false;
    for _ in 0..io::RETRIES {
        io::write(&io::DEVICE_DATA_PORT, cmd);
        match io::read(&DEVICE_DATA_PORT) {
            Some(io::RESEND) => continue,
            Some(io::ACK) => {
                ok = true;
                break;
            },
            Some(unknown) => return Err(Ps2Error::UnexpectedResponse(unknown)),
            None => return Err(Ps2Error::ExpectedResponse),
        }
    }

    if !ok {
        return Err(Ps2Error::RetriesExceeded);
    }

    for _ in 0..io::RETRIES {
        io::write(&io::DEVICE_DATA_PORT, data);
        match io::read(&DEVICE_DATA_PORT) {
            Some(io::RESEND) => continue,
            Some(io::ACK) => return Ok(()), // TODO check dis
            Some(unknown) => return Err(Ps2Error::UnexpectedResponse(unknown)),
            None => return Err(Ps2Error::ExpectedResponse),
        }
    }

    Ok(())
}


struct GenericDevice {
    port: DevicePort
}

// TODO
impl GenericDevice {

}

pub struct Keyboard {
    port: DevicePort
}

/// Represents a PS/2 scanset
#[allow(dead_code)] // Dead variants for completeness
#[repr(u8)]
pub enum Scanset {
    One = 1,
    Two = 2,
    Three = 3
}

/// Represents a PS/2 scancode received from the device
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Scancode {
    pub code: u8,
    pub extended: bool,
    pub make: bool,
}

impl Scancode {
    /// Constructs a new [Ps2Scancode]
    fn new(scancode: u8, extended: bool, make: bool) -> Self {
        Scancode { code: scancode, extended, make }
    }
}

impl Keyboard {
    pub const fn new(port: DevicePort) -> Self {
        Keyboard { port }
    }

    /// Reads a single scancode from this PS/2 keyboard, returning None if the next byte to be read
    /// from PS/2 is not for the keyboard, or if there is no available byte to be read.
    // TODO this ^ could create a situation where the keyboard is waiting for data from the mouse to
    // be read so that it can read
    /// # Examples
    ///
    /// ```rust,no_run
    /// let device = drivers::ps2::CONTROLLER.device(drivers::ps2::DevicePort::Keyboard);
    /// let mut keyboard = Ps2Keyboard::new(device);
    ///
    /// if let Some(scancode) = keyboard.read_scancode()? {
    ///     print!(scancode);
    /// }
    /// ```
    pub fn read_scancode(&self) -> Result<Option<Scancode>, Ps2Error> {
        // TODO when check state
        // TODO illegal state unrepresentable?
//        if self.state()? != DeviceState::Enabled {
//            return Err(Ps2Error::DeviceDisabled)
//        }

        if io::can_read() && io::can_read_keyboard() {
            return Ok(None)
        }

        let mut make = true;
        let mut extended = false;

        // Get all scancode modifiers, and return when the actual scancode is received
        let scancode = loop {
            let data = io::read(&io::DEVICE_DATA_PORT)?;
            match data {
                0xE0 | 0xE1 => extended = true,
                0xF0 => make = false,
                data => {
                    break data;
                },
            }
        };

        // If scancode is present, return it with modifiers
        return Ok(if scancode != 0 {
            Some(Scancode::new(scancode, extended, make))
        } else {
            None
        });
    }

    pub fn set_scanset(&self, scanset: Scanset) -> Result<(), Ps2Error> {
        unimplemented!() // TODO
    }
}

impl Device for Keyboard {
    fn port(&self) -> DevicePort {
        self.port
    }
}

pub struct Mouse {
    port: DevicePort
}

impl Mouse {
    pub const fn new(port: DevicePort) -> Self {
        Mouse { port }
    }
}

impl Device for Mouse {
    fn port(&self) -> DevicePort {
        self.port
    }
}