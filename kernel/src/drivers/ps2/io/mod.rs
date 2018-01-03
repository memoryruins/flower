//! Internal, low-level IO with PS/2 ports

// TODO should mut for ioport stuff?
use io::IOPort;

/// The number of iterations before assuming no data to be read. Should be changed to a timeout as of #26
pub static WAIT_TIMEOUT: u32 = 1000000; // TODO configure
pub static RETRIES: u8 = 4;

pub const RESEND: u8 = 0xFE;
pub const ACK: u8 = 0xFA;
pub const KEYBOARD_DETECTION_ERROR: (u8, u8) = (0x0, 0xFF);
pub const KEYBOARD_SELF_TEST_PASSED: u8 = 0xAA;
pub const KEYBOARD_SELF_TEST_FAILED: (u8, u8) = (0xFC, 0xFD); // TODO should check for this on keyboard acquire
pub const KEYBOARD_ECHO_REPLY: u8 = 0xEE;

// TODO rename
pub static DEVICE_DATA_PORT: IOPort = unsafe { IOPort::new(0x60) };
pub static STATUS_COMMAND_PORT: IOPort = unsafe { IOPort::new(0x64) };

pub mod command;

bitflags! {
    struct StatusFlags: u8 {
        /// If the output buffer from the controller is full (data can be read)
        const OUTPUT_FULL = 1 << 0;
        /// If the input buffer to the controller is full (data cannot be written)
        const INPUT_FULL = 1 << 1;
        /// If the current output from the controller is from the second port
        const OUTPUT_PORT_2 = 1 << 5;
    }
}

/// Writes to the given port, or waits until available
// TODO async? Don't just busy loop
// TODO timeout
pub fn write(port: &IOPort, value: u8) {
    loop {
        // Check if the input status bit is empty
        if can_write() {
            port.write(value);
        }
    }
}

/// Reads from the given port, returning a value if something could be read, otherwise `None`
// TODO should be err?
pub fn read(port: &IOPort) -> Option<u8> {
    for _ in 0..WAIT_TIMEOUT {
        // Check if the output status bit is full
        if can_read() {
            return Some(port.read());
        }
    }

    None
}

/// Flushes the controller's output buffer, discarding any bytes in the buffer
pub fn flush_output()  {
    // Read until the output status bit is empty
    while can_read() {
        DEVICE_DATA_PORT.read();
    }
}

/// Reads from the status port and returns the flags
fn read_status() -> StatusFlags {
    StatusFlags::from_bits_truncate(STATUS_COMMAND_PORT.read())
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
    !read_status().contains(StatusFlags::OUTPUT_PORT_2)
}

/// Returns true if output port bit is 1, meaning the next data will be read from the mouse
pub fn can_read_mouse() -> bool {
    read_status().contains(StatusFlags::OUTPUT_PORT_2)
}
