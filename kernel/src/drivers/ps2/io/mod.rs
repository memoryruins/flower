//! Internal, low-level IO with PS/2 ports

pub mod command;

use super::Ps2Error;

use self::command::controller;
use io::SynchronizedPort;

pub const RESEND: u8 = 0xFE;
pub const ACK: u8 = 0xFA;
pub const CONTROLLER_TEST_SUCCESS: u8 = 0x55;
pub const ECHO: u8 = 0xEE;

/// The amount of iterations to resend a device command before returning an error
pub const COMMAND_RETRIES: usize = 16;

/// The amount of iterations to wait for IO access before terminating and returning a PS/2 error
pub const IO_ITERATIONS: usize = 1000000;

/// Port used to send data to the controller, and for the controller to return responses
pub static DATA_PORT: SynchronizedPort<u8> = unsafe { SynchronizedPort::new(0x60) };
/// Port used to check controller status and to send commands to the controller
pub static CONTROLLER_PORT: SynchronizedPort<u8> = unsafe { SynchronizedPort::new(0x64) };

bitflags! {
    struct StatusFlags: u8 {
        /// If the output buffer from the controller is full (data can be read)
        const OUTPUT_FULL = 1 << 0;
        /// If the input buffer to the controller is full (data cannot be written)
        const INPUT_FULL = 1 << 1;
        /// If the current output from the controller is from the second port
        const MOUSE_OUTPUT_FULL = 1 << 5;
    }
}

/// Sends a controller command
pub fn command(command: controller::Command) {
    flush_output();
    write_blocking(&CONTROLLER_PORT, command as u8);
}

/// Sends a controller command with data
pub fn command_data(command: controller::DataCommand, data: u8) {
    flush_output();
    write_blocking(&CONTROLLER_PORT, command as u8);
    write_blocking(&DATA_PORT, data as u8);
}

/// Writes to the given port, or returns an error if write unavailable
pub fn write(port: &SynchronizedPort<u8>, value: u8) -> Result<(), Ps2Error> {
    // Check if the input status bit is empty
    if can_write() {
        port.write(value);
        Ok(())
    } else {
        Err(Ps2Error::WriteUnavailable)
    }
}

/// Writes to the given port, and blocks until available. Panics if the output status bit is never
/// unset within a time because that should never happen.
pub fn write_blocking(port: &SynchronizedPort<u8>, value: u8) {
    // Iterate until maximum iterations reached or write available
    for _ in 0..IO_ITERATIONS {
        if can_write() {
            port.write(value);
            return;
        }
    }
    panic!("Writing to PS/2 controller took too long!");
}

/// Reads from the given port, returning a value if any data was present, otherwise returns `None`
/// Note that you may receive `None` even when expecting a response, as the PS/2 controller may
/// not have had time to respond yet.
pub fn read(port: &SynchronizedPort<u8>) -> Option<u8> {
    // Check if the output status bit is full
    if can_read() {
        return Some(port.read());
    }
    None
}

/// Reads from the given port, or blocks until a response has been sent back. Returns `None` if
/// timeout is exceeded while awaiting a response.
pub fn read_blocking(port: &SynchronizedPort<u8>) -> Option<u8> {
    // Iterate until maximum iterations reached or response available
    for _ in 0..IO_ITERATIONS {
        if can_read() {
            return Some(port.read());
        }
    }
    None
}

/// Flushes the controller's output buffer, discarding any bytes in the buffer
pub fn flush_output() {
    // Read until the output status bit is empty
    while can_read() {
        let _ = DATA_PORT.read();
    }
}

/// Reads from the status port and returns the flags
fn read_status() -> StatusFlags {
    StatusFlags::from_bits_truncate(CONTROLLER_PORT.read())
}

/// Returns true if the write status bit is 0
fn can_write() -> bool {
    !read_status().contains(StatusFlags::INPUT_FULL)
}

/// Returns true if the read status bit is 1
pub fn can_read() -> bool {
    read_status().contains(StatusFlags::OUTPUT_FULL)
}

/// Returns true if output port bit is 0, meaning the next data will be read from the keyboard
pub fn can_read_keyboard() -> bool {
    !read_status().contains(StatusFlags::MOUSE_OUTPUT_FULL)
}

/// Returns true if output port bit is 1, meaning the next data will be read from the mouse
pub fn can_read_mouse() -> bool {
    read_status().contains(StatusFlags::MOUSE_OUTPUT_FULL)
}
