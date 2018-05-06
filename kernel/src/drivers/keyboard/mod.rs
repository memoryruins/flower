//! # Keyboard Driver
//!
//! The keyboard driver handles all keyboard related functionality, intended to support both PS/2 and USB.
//! Currently, only PS/2 support has been implemented through the use of the PS/2 driver.
//!
//! The driver is event based, and events are received through the `read_event` method, which blocks until an event is received.
//! The event contains the keycode pressed, which can be compared to `keymap::codes`, an optional `char`, the type of press, and various modifier flags.
//!
//! # Examples
//!
//! ```rust,no_run
//! let device = drivers::ps2::CONTROLLER.device(drivers::ps2::DevicePort::Keyboard);
//! let mut keyboard = Ps2Keyboard::new(device);
//!
//! keyboard.enable()?;
//! loop {
//!     let event = keyboard.read_event()?;
//!     handle_event(event);
//! }
//! ```

// TODO: Redo all examples

use core::convert::{TryFrom, TryInto};
use drivers::ps2::{self, Ps2Error};

pub mod keymap;

bitflags! {
    pub struct ModifierFlags: u8 {
        /// If a CTRL modifier is active
        const CTRL = 1 << 0;
        /// If an ALT modifier is active
        const ALT = 1 << 1;
        /// If a SHIFT modifier is active
        const SHIFT = 1 << 2;
    }
}

impl ModifierFlags {
    /// Creates `ModifierFlags` from the given contained booleans
    ///
    /// # Examples
    ///
    /// ```rust
    /// let modifiers = ModifierFlags::from_modifiers(true, true, true);
    /// assert_eq!(modifiers, ModifierFlags::CTRL | ModifierFlags::ALT | ModifierFlags::SHIFT);
    /// ```
    fn from_modifiers(ctrl: bool, alt: bool, shift: bool) -> Self {
        let mut flags = ModifierFlags::empty();
        flags.set(ModifierFlags::CTRL, ctrl);
        flags.set(ModifierFlags::ALT, alt);
        flags.set(ModifierFlags::SHIFT, shift);
        flags
    }
}

pub type Keycode = u8;

/// Contains data relating to a key press event
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KeyEvent {
    pub keycode: Keycode,
    pub char: Option<char>,
    pub event_type: KeyEventType,
    pub modifiers: ModifierFlags,
}

/// The type of key event that occurred
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum KeyEventType {
    /// When the key is initially pressed
    Make,
    /// When the key is released
    Break,
    /// When the key is held down, and a repeat is fired
    Repeat,
}

/// Interface to a generic keyboard.
pub trait Keyboard {
    type Error;

    /// Polls the device for a new key state event, or returns `None` if none have occurred since the last poll.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let device = drivers::ps2::CONTROLLER.device(drivers::ps2::DevicePort::Keyboard);
    /// let mut keyboard = Ps2Keyboard::new(device);
    ///
    /// if let Some(event) = keyboard.read_event()? {
    ///     println!("Event occurred for char: {}", event.char.unwrap_or(' '));
    /// }
    /// ```
    // TODO: This should eventually use interrupts and hold a queue
    fn read_event(&mut self) -> Result<Option<KeyEvent>, Self::Error>;

    /// Returns `true` if the given keycode is currently being pressed
    ///
    /// ```rust,no_run
    /// let device = drivers::ps2::CONTROLLER.device(drivers::ps2::DevicePort::Keyboard);
    /// let mut keyboard = Ps2Keyboard::new(device);
    ///
    /// if keyboard.pressed(keymap::codes::LEFT_SHIFT) {
    ///     println!("Left shift pressed");
    /// } else {
    ///     println!("Left shift not pressed");
    /// }
    /// ```
    fn pressed(&self, keycode: Keycode) -> bool;
}

/// Handles interface to a PS/2 keyboard, if available
pub struct Ps2Keyboard {
    key_states: [bool; 0xFF],
}

impl Ps2Keyboard {
    /// Creates a new Ps2Keyboard from the given PS/2 device
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let device = drivers::ps2::CONTROLLER.device(drivers::ps2::DevicePort::Keyboard);
    /// let mut keyboard = Ps2Keyboard::new(device);
    /// ```
    pub fn new() -> Self {
        ps2::CONTROLLER.lock().set_keyboard_change_listener(Ps2Keyboard::on_keyboard_change);
        Ps2Keyboard {
            key_states: [false; 0xFF],
        }
    }

    fn on_keyboard_change(keyboard: ps2::device::keyboard::Keyboard) -> Result<(), Ps2Error> {
        keyboard.set_scanset(ps2::device::keyboard::Scanset::Two)
    }

    /// Creates a [KeyEvent] from the given scancode and key state
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let scancode = ps2::device::keyboard::Scancode::new(0x15, false, true);
    /// let event = keyboard.create_event(&scancode).unwrap();
    /// assert_eq!(event.keycode, keymap::codes::Q);
    /// assert_eq!(event.char, Some('q'));
    /// assert_eq!(event.event_type, KeyEventType::Make);
    /// ```
    fn create_event(&self, scancode: &ps2::device::keyboard::Scancode) -> Option<KeyEvent> {
        let ctrl = self.pressed(keymap::codes::LEFT_CONTROL) || self.pressed(keymap::codes::RIGHT_CONTROL);
        let alt = self.pressed(keymap::codes::LEFT_ALT) || self.pressed(keymap::codes::RIGHT_ALT);
        let shift = self.pressed(keymap::codes::LEFT_SHIFT) || self.pressed(keymap::codes::RIGHT_SHIFT);
        let modifiers = ModifierFlags::from_modifiers(ctrl, alt, shift);

        if let Ok(keycode) = (*scancode).try_into() {
            let char = keymap::get_us_qwerty_char(keycode)
                .map(|chars| if shift {
                    chars.1
                } else {
                    chars.0
                });

            // If the key was already pressed and make was sent, this is a repeat event
            let event_type = match scancode.make {
                true if self.pressed(keycode) => KeyEventType::Repeat,
                true => KeyEventType::Make,
                false => KeyEventType::Break,
            };

            return Some(KeyEvent { keycode, char, event_type, modifiers });
        }

        None
    }
}

impl Keyboard for Ps2Keyboard {
    type Error = Ps2Error;

    fn read_event(&mut self) -> Result<Option<KeyEvent>, Ps2Error> {
        Ok(ps2::CONTROLLER.lock().keyboard()?.read_scancode()?.map(|scancode| {
            let event = self.create_event(&scancode);
            if let Some(event) = event {
                self.key_states[event.keycode as usize] = scancode.make;
            }
            event
        }).unwrap_or(None))
    }

    fn pressed(&self, keycode: Keycode) -> bool {
        *self.key_states.get(keycode as usize).unwrap_or(&false)
    }
}

pub enum UnknownScancode {
    UnknownPlainScancode(u8),
    UnknownExtendedScancode(u8),
}

impl TryFrom<ps2::device::keyboard::Scancode> for Keycode {
    type Error = UnknownScancode;

    /// Gets the Flower keycode for this scancode
    ///
    /// # Examples
    ///
    /// ```rust
    /// let scancode = ps2::device::keyboard::Scancode::new(0x01, false, true);
    /// assert_eq!(scancode.keycode(), Some(keymap::codes::KEY_F9));
    /// ```
    fn try_from(scancode: ps2::device::keyboard::Scancode) -> Result<Keycode, UnknownScancode> {
        use self::UnknownScancode::*;

        let converted = if !scancode.extended {
            keymap::get_code_ps2_set_2(scancode.code)
        } else {
            keymap::get_extended_code_ps2_set_2(scancode.code)
        };

        match (converted, scancode.extended) {
            (Some(keycode), _) => Ok(keycode),
            (None, false) => Err(UnknownPlainScancode(scancode.code)),
            (None, true) => Err(UnknownExtendedScancode(scancode.code)),
        }
    }
}
