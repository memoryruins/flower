use core::{convert::From, option};
use self::device::{DevicePort, InternalDevice, Keyboard, Mouse, PortType};
use self::io::command;
use spin::Mutex;

pub mod io;
pub mod device;

pub static PORTS: PortHolder = PortHolder::new();

/// Represents an error returned by PS/2
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Ps2Error {
    RetriesExceeded,
    DeviceUnavailable,
    DeviceDisabled,
    UnexpectedResponse(u8),
    /// When a response was expected but not received
    ExpectedResponse,
    /// Returned when the write status bit is set, meaning write should be retried until available
    WriteUnavailable,
    ControllerTestFailed,
    /// Returned when an error occurs during initialization
    InitializationError(InitializationError),
    WrongDeviceType,
}

/// Represents an error during PS/2 initialization
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InitializationError {
    ControllerTestFailed(u8),
}

impl From<option::NoneError> for Ps2Error {
    fn from(_: option::NoneError) -> Self {
        Ps2Error::ExpectedResponse
    }
}

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

// TODO: We need to properly mark ports as dirty at intervals or when unexpected data is received (or not)
pub struct Controller<'a> {
    devices: (Option<InternalDevice<'a>>, Option<InternalDevice<'a>>),
}

impl<'a> Controller<'a> {
    pub fn new() -> Self {
        Controller {
            devices: (None, None),
        }
    }

    pub fn setup(&mut self) -> Result<(), Ps2Error> {
        // TODO: Check the PS/2 controller is present

        info!("ps2c: initializing");

        // Reset the ports, we haven't tested if they exist yet
        PORTS.set((None, None));

        // Call the disable command for both ports
        PortType::Port1.disable();
        PortType::Port2.disable();

        debug!("ps2c: disabled devices");

        // Flush output buffer to make sure there's nothing queued up from the devices
        io::flush_output();

        let config = self.initialize_config()?;
        self.test_controller()?;

        // Test if both of the PS/2 ports are present for this device
        let available_ports = self.test_ports(config)?;
        match available_ports {
            (false, _) => warn!("ps2c: first port not present"),
            (_, false) => warn!("ps2c: second port not present"),
            _ => (),
        }

        // Enable interrupts again now that everything is set up
        let mut config = self.config()?;
        config.set(ConfigFlags::PORT_INTERRUPT_1, true);
        config.set(ConfigFlags::PORT_INTERRUPT_2, true);
        self.set_config(config);

        // Call the reset on all devices, including ones not considered present
        PORTS.perform_port(|p| p.reset())?;

        debug!("ps2c: reset devices");

        // Make sure no initialization left anything in the output buffer
        io::flush_output();

        Ok(())
    }

    /// Initializes the config for this controller
    fn initialize_config(&self) -> Result<ConfigFlags, Ps2Error> {
        // Read the config from the controller
        let mut config = self.config()?;

        // Set all required config flags
        config.set(ConfigFlags::PORT_INTERRUPT_1, false);
        config.set(ConfigFlags::PORT_INTERRUPT_2, false);
        config.set(ConfigFlags::PORT_TRANSLATION_1, false);

        // Write the updated config back to the controller
        self.set_config(config);

        info!("ps2c: initialized config");

        Ok(config)
    }

    /// Tests this controller, returning `Ok` if successful, or an `Ps2Error:InitializationError` if
    /// it failed.
    fn test_controller(&mut self) -> Result<(), Ps2Error> {
        io::command(command::controller::Command::TestController);

        let test_result = io::read_blocking(&io::DATA_PORT)?;
        if test_result == io::CONTROLLER_TEST_SUCCESS {
            debug!("ps2c: tested controller");
            Ok(())
        } else {
            Err(Ps2Error::InitializationError(InitializationError::ControllerTestFailed(test_result)))
        }
    }

    /// Test which ports are supported by this controller
    fn test_ports(&mut self, config: ConfigFlags) -> Result<(bool, bool), Ps2Error> {
        let mut config = config;

        // Check if controller supports the second port
        if config.contains(ConfigFlags::PORT_CLOCK_2) {
            PortType::Port2.enable();
            config = self.config()?;
            PortType::Port2.disable();
        }

        // Test both ports
        let first_supported = PortType::Port1.test()?;
        let second_supported = !config.contains(ConfigFlags::PORT_CLOCK_2) && PortType::Port2.test()?;

        config.set(ConfigFlags::PORT_CLOCK_1, true);
        config.set(ConfigFlags::PORT_CLOCK_2, true);

        let available_ports = (first_supported, second_supported);
        PORTS.set((
            if available_ports.0 { Some(DevicePort::new(PortType::Port1)) } else { None },
            if available_ports.1 { Some(DevicePort::new(PortType::Port2)) } else { None },
        ));

        info!("ps2c: tested ports");

        Ok(available_ports)
    }

    pub fn config(&self) -> Result<ConfigFlags, Ps2Error> {
        io::command(command::controller::Command::ReadConfig);

        let config_data = io::read_blocking(&io::DATA_PORT)?;
        Ok(ConfigFlags::from_bits_truncate(config_data))
    }

    pub fn set_config(&self, config: ConfigFlags) {
        io::command_data(command::controller::DataCommand::WriteConfig, config.bits())
    }

    pub fn keyboard(&mut self) -> Result<Keyboard<'a>, Ps2Error> {
        self.refresh_state()?;

        // Optionally map both of the devices to a keyboard
        let keyboards = self.map_device(|d| d.as_keyboard());
        keyboards.0.or(keyboards.1).ok_or_else(|| Ps2Error::DeviceUnavailable)
    }

    pub fn mouse(&mut self) -> Result<Mouse<'a>, Ps2Error> {
        self.refresh_state()?;

        // Optionally map both of the devices to a mouse
        let mice = self.map_device(|d| d.as_mouse());
        mice.0.or(mice.1).ok_or_else(|| Ps2Error::DeviceUnavailable)
    }

    fn refresh_state(&mut self) -> Result<(), Ps2Error> {
        if refresh_port(&PORTS.0, &mut self.devices.0)? {
            self.devices.0.as_mut().map(|d| d.init());
        }

        if refresh_port(&PORTS.1, &mut self.devices.1)? {
            self.devices.1.as_mut().map(|d| d.init());
        }

        Ok(())
    }

    fn map_device<F, R>(&self, f: F) -> (Option<R>, Option<R>)
        where F: Fn(&InternalDevice<'a>) -> Option<R> + Clone
    {
        (
            self.devices.0.as_ref().and_then(f.clone()),
            self.devices.1.as_ref().and_then(f.clone()),
        )
    }
}

fn refresh_port<'a>(port_mutex: &'static Mutex<Option<DevicePort>>, device: &mut Option<InternalDevice<'a>>) -> Result<bool, Ps2Error> {
    if let Some(ref mut port) = *port_mutex.lock() {
        if port.is_dirty() {
            debug!("ps2c: refreshing {:?} device", port.port_type);
            port.set_dirty(false);

            let ping = port.ping()?;
            *device = if ping {
                Some(InternalDevice::new(port.port_type, &port_mutex))
            } else {
                None
            };

            return Ok(ping);
        }
    }

    Ok(false)
}

pub struct PortHolder(Mutex<Option<DevicePort>>, Mutex<Option<DevicePort>>);

impl PortHolder {
    const fn new() -> Self {
        PortHolder(Mutex::new(None), Mutex::new(None))
    }

    fn set(&self, ports: (Option<DevicePort>, Option<DevicePort>)) {
        *self.0.lock() = ports.0;
        *self.1.lock() = ports.1;
    }

    fn perform_port<'b, F, R, E>(&self, f: F) -> Result<(Option<R>, Option<R>), E>
        where F: Fn(&mut DevicePort) -> Result<R, E>
    {
        Ok((
            match self.0.lock().as_mut() {
                Some(ref mut port) => Some(f(port)?),
                None => None,
            },
            match self.1.lock().as_mut() {
                Some(ref mut port) => Some(f(port)?),
                None => None,
            },
        ))
    }
}
