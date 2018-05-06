use drivers::ps2::{device::{self, Device, DevicePort}, Ps2Error};
use drivers::ps2::io::command::device::mouse::{Command, DataCommand};
use spin::Mutex;

#[derive(Debug)]
pub struct Mouse<'a> {
    internal: &'a Mutex<Option<DevicePort>>,
}

impl<'a> Mouse<'a> {
    pub(super) const fn new(internal: &'a Mutex<Option<DevicePort>>) -> Self {
        Mouse { internal }
    }
}

impl<'a> Device for Mouse<'a> {
    fn port(&self) -> &Mutex<Option<DevicePort>> { self.internal }

    fn is_mouse(&self) -> bool { true }

    fn is_keyboard(&self) -> bool { false }
}

impl<'a> Mouse<'a> {
    fn command_mouse(&self, cmd: Command) -> Result<(), Ps2Error> {
        if self.is_enabled() {
            let port = self.port().lock();
            device::command_raw(port.as_ref().ok_or(Ps2Error::DeviceUnavailable)?, cmd as u8)
        } else {
            Err(Ps2Error::DeviceDisabled)
        }
    }

    fn command_mouse_data(&self, cmd: DataCommand, data: u8) -> Result<(), Ps2Error> {
        if self.is_enabled() {
            let port = self.port().lock();
            device::command_raw_data(port.as_ref().ok_or(Ps2Error::DeviceUnavailable)?, cmd as u8, data)
        } else {
            Err(Ps2Error::DeviceDisabled)
        }
    }
}
