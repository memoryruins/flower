//! # PS/2 Driver
//!
//! The PS/2 driver provides interface into the PS/2 controller, allowing access to devices using
//! this protocol. The controller is accessed through the static `CONTROLLER`.
//!
//! The [Controller] handles all interface with the controller for devices.
//! For it to be initialized, `initialize` must be called on it. This sets up all attached devices.
//!
//! The [Device] handles interface to a single PS/2 device. Its state can be checked and toggled
//! through `enable` and `disable`.
//!
//! Devices can be obtained from the controller through `device(DevicePort)` or functions for keyboard
//! and mouse

// TODO document
// TODO no-run examples & rust highlight

mod io;
mod device;

// TODO check is right
pub use self::device::*;

use core::{option, convert::From};
use spin::Mutex;
use self::io::{STATUS_COMMAND_PORT, DEVICE_DATA_PORT, command::controller::*};

pub static CONTROLLER: Mutex<Controller> = Mutex::new(Controller::new());

bitflags! {
    pub struct ConfigFlags: u8 {
        /// Whether interrupts for Port 1 are enabled
        const PORT_INTERRUPT_1 = 1 << 0;
        /// Whether interrupts for Port 2 are enabled
        const PORT_INTERRUPT_2 = 1 << 1;
        /// Whether the clock for Port 1 is disabled
        const PORT_CLOCK_1 = 1 << 4;
        /// Whether the clock for Port 2 is disabled
        const PORT_CLOCK_2 = 1 << 5;
        /// Whether the controller will transform scan set 2 to scan set 1
        const PORT_TRANSLATION_1 = 1 << 6;
    }
}

/// Represents an error returned by PS/2
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Ps2Error {
    // TODO configure retries?
    RetriesExceeded,
    DeviceUnavailable,
    DeviceDisabled, // TODO how to know
    UnexpectedResponse(u8),
    /// When a response was expected but not received
    ExpectedResponse,
}

impl From<option::NoneError> for Ps2Error {
    fn from(_: option::NoneError) -> Self {
        Ps2Error::ExpectedResponse
    }
}

/// Represents the PS2 master controller
pub struct Controller {
    devices: DevicesInner,
}

/// Represents the inner devices stored by the controller
struct DevicesInner {
    keyboard: Option<Keyboard>, // TODO should make sure that if device is in it is not none
    mouse: Option<Mouse>,
}

impl DevicesInner {
    const fn new() -> Self {
        DevicesInner {
            keyboard: None,
            mouse: None,
        }
    }

    /// Maps the closure onto the keyboard and then the device, returning the returns of the closure
    /// mapped onto the keyboard first and then the closure mapped onto the second, or none if the
    /// device is not present.
    fn map<'a, F, R>(&mut self, f: F) -> (Option<R>, Option<R>)
        where F: Fn(&(Device + 'a)) -> R
    {
        (
            self.keyboard.as_ref().map(|d| f(d)),
            self.mouse.as_ref().map(|d| f(d))
        )
    }

    /// Maps the closure onto the keyboard and then the device (mutable), returning the returns of
    /// the closure mapped onto the keyboard first and then the closure mapped onto the second, or
    /// none if the device is not present
    // TODO much clearer in doc
    fn map_mut<'a, F, R>(&mut self, mut f: F) -> (Option<R>, Option<R>)
        where F: FnMut(&mut (Device + 'a)) -> R
    {
        (
            self.keyboard.as_mut().map(|d| f(d)),
            self.mouse.as_mut().map(|d| f(d))
        )
    }



}

impl Controller {

    /// Creates a new PS2 controller
    const fn new() -> Self {
        Controller {
            devices: DevicesInner::new(),
        }
    }

    // TODO merge with `new`?
    /// Initializes this PS2 controller
    ///
    /// # Shortcircuiting
    ///
    /// Shortcircuits and returns the first error from device reset.
    pub fn initialize(&mut self) -> Result<(), Ps2Error> {
        info!("ps2c: initializing");

        // TODO ? v v v
        self.devices.map(|d| d.disable());
        println!("ps2c: disabled devices");

        io::flush_output();

        self.initialize_config()?;

        if !self.test_controller()? {
            error!("ps2c: controller test failed");
        }

        debug!("ps2c: testing devices");
        match self.test_devices()? {
            (false, _) => warn!("ps2c: first device not supported"),
            (_, false) => warn!("ps2c: second device not supported"),
            _ => (),
        }

        // Check if any devices are available
        if self.reset_devices()? > 0 {
            debug!("ps2c: prepared devices");
        // TODO y?

//        println!("ps2c: testing devices");
//        match self.test_devices()? {
//            (false, _) => println!("ps2c: first device not supported"),
//            (_, false) => println!("ps2c: second device not supported"),
//            _ => (),
//        }

        // Reset devices and propagate errors up
        self.reset_devices().into_iter()
            .filter_map(|x| *x)
            .collect::<Result<(), Ps2Error>>()?;

        // Check if any devices are available
        if self.discover_devices()? > 0 {
            println!("ps2c: prepared devices");
        } else {
            info!("ps2c: detected no available devices");
        }

        io::flush_output();

        Ok(())
    }

    /// Reads the config from the PS2 controller
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let controller = ps2::CONTROLLER.lock();
    /// let config = controller.read_config();
    /// ```
    pub fn config(&self) -> Result<ConfigFlags, Ps2Error> {
        let read = self.command_ret(ControllerReturnCommand::ReadConfig)?;

        Ok(ConfigFlags::from_bits_truncate(read))
    }

    /// Writes the given config to the PS2 controller
    ///
    /// # Examples
    /// ```rust,no_run
    /// let controller = ps2::CONTROLLER.lock();
    /// let config = ConfigFlags::from_bits_truncate(0xbeef);
    /// controller.lock().write_config(config);
    /// ```
    pub fn set_config(&self, config: ConfigFlags) {
        self.command_data(ControllerDataCommand::WriteConfig, config.bits())
    }

    /// Gets the device for the given port if available
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let controller = ps2::CONTROLLER.lock();
    /// let device = controller.device(DevicePort::Keyboard).unwrap();
    /// ```
    pub fn device(&self, port: DevicePort) -> Option<&Device> {
        if let Some(ref keyboard) = self.devices.keyboard {
            if keyboard.port() == port {
                return Some(keyboard as &_)
            }
        }

        if let Some(ref mouse) = self.devices.mouse {
            if mouse.port() == port {
                return Some(mouse as &_)
            }
        }

        None
    }

    pub fn device_mut(&mut self, port: DevicePort) -> Option<&mut Device> {
        if let Some(keyboard) = self.devices.keyboard.as_mut() {
            if keyboard.port() == port {
                return Some(keyboard);
            }
        }

<<<<<<< HEAD
        debug!("ps2c: initialized config");
=======
        if let Some(mouse) = self.devices.mouse.as_mut() {
            if mouse.port() == port {
                return Some(mouse);
            }
        }
>>>>>>> Ps2 stuff

        None
    }

    /// Gets the PS/2 keyboard if available
    pub fn keyboard(&self) -> Option<&Keyboard> {
        // TODO check if available
        self.devices.keyboard.as_ref()
    }

    /// Gets the PS/2 keyboard mutably if available
    pub fn keyboard_mut(&mut self) -> Option<&mut Keyboard> {
        self.devices.keyboard.as_mut()
    }

    /// Gets the PS/2 mouse if available
    pub fn mouse(&self) -> Option<&Mouse> {
        self.devices.mouse.as_ref()
    }

    /// Gets the PS/2 mouse if available
    pub fn mouse_mut(&mut self) -> Option<&mut Mouse> {
        self.devices.mouse.as_mut()
    }

    /// Initializes the config for this controller
    fn initialize_config(&self) -> Result<(), Ps2Error> {
        // Read the config from the controller
        let mut config = self.config()?;

        // Set all required config flags
        config.set(ConfigFlags::PORT_INTERRUPT_1, false);
        config.set(ConfigFlags::PORT_INTERRUPT_2, false);
        config.set(ConfigFlags::PORT_TRANSLATION_1, false);

        // Write the updated config back to the controller
        self.set_config(config);

        println!("ps2c: initialized config");

        Ok(())
    }

    /// Tests this controller, returning `None` if no reply from the controller
    fn test_controller(&mut self) -> Result<bool, Ps2Error> {
        Ok(self.command_ret(ControllerReturnCommand::TestController)? == 0x55)
    }

    // TODO when should this be called?
    // TODO eventually use interrupts?
    fn discover_devices(&mut self) -> Result<u8, Ps2Error> {
        // TODO debug_assert! that result isn't > 2 or < 2
        unimplemented!()
    }

    /// Resets all devices
    fn reset_devices(&mut self) -> [Option<Result<(), Ps2Error>>; 2] {
        let results = self.devices.map(|d| d.reset());
        [results.0, results.1]
    }

    /// Sends a controller command without a return
    fn command(&self, cmd: ControllerCommand) { // TODO Does this even need to be a function
        io::write(&io::STATUS_COMMAND_PORT, cmd as u8); // TODO prolly because strict cmd bound
        // TODO this but as helper for device/mod.rs
    }

<<<<<<< HEAD
    /// Sends a command for this PS2 device with data and returns a result
    #[allow(dead_code)] // To be used by drivers interfacing with PS/2
    pub fn command_data(&mut self, cmd: DeviceDataCommand, data: u8) -> Result<u8, Ps2Error> {
        if self.state != DeviceState::Unavailable {
            self.command_raw(cmd as u8).and_then(|result| match result {
                ACK => {
                    io::DATA_PORT.with_lock(|mut data_port| {
                        io::write(&mut data_port, data as u8)?;
                        io::read(&mut data_port)
                    })
                }
                _ => Ok(result)
            })
        } else {
            Err(Ps2Error::DeviceUnavailable)
        }
    }

    /// Sends a raw command code to this device
    fn command_raw(&mut self, cmd: u8) -> Result<u8, Ps2Error> {
        if self.state != DeviceState::Unavailable {
            // If second PS2 port, send context switch command
            if self.port == DevicePort::Mouse {
                commands::send(ControllerCommand::WriteInputPort2)?;
            }

            io::DATA_PORT.with_lock(|mut data_port| {
                for _ in 0..4 {
                    io::write(&mut data_port, cmd)?;
                    match io::read(&mut data_port) {
                        Ok(RESEND) => continue,
                        result => return result,
                    }
                }

                Err(Ps2Error::NoData)
            })
        } else {
            Err(Ps2Error::DeviceUnavailable)
        }
=======
    /// Sends a controller command with data and without a return
    fn command_data(&self, cmd: ControllerDataCommand, data: u8) {
        io::write(&io::STATUS_COMMAND_PORT, cmd as u8);
        io::write(&io::DEVICE_DATA_PORT, data as u8);
    }

    /// Sends a controller command with a return
    fn command_ret(&self, cmd: ControllerReturnCommand) -> Result<u8, Ps2Error> {
        io::write(&STATUS_COMMAND_PORT, cmd as u8);
        Ok(io::read(&DEVICE_DATA_PORT)?)
>>>>>>> Ps2 stuff
    }
}

