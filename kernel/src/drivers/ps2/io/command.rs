//! Enums representing PS2 commands

// TODO doc

/// Enums representing controller commands
pub mod controller {
    // TODO add more controller cmd's
    /// Represents a PS2 controller command without a return value
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum ControllerCommand {
        DisablePort2 = 0xA7, // done
        EnablePort2 = 0xA8, // done
        DisablePort1 = 0xAD, // done
        EnablePort1 = 0xAE, // done
        WriteInputPort2 = 0xD4, // done
    }

    /// Represents a PS2 controller command with a return value
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum ControllerReturnCommand {
        ReadConfig = 0x20, // done
        TestController = 0xAA, // done
        TestPort1 = 0xAB, // done
        TestPort2 = 0xA9, // done
        IdentifyDevice = 0xF2, // TODO
    }

    /// Represents a PS2 controller command with a data value
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum ControllerDataCommand {
        WriteConfig = 0x60, // done
    }
}

/// Enums representing PS2 device commands
pub mod device {

    /// Represents a general PS2 device command without a return and without additional data
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum DeviceCommand {
        // TODO doc
        EnableScanning = 0xF4, // done
        DisableScanning = 0xF5, // done // TODO may reset defautls?? WTF?
        SetDefaults = 0xF6, // done
        Reset = 0xFF, // done
    }

    /// Represents a general PS2 device command without a return and with additional data
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum DeviceReturnCommand {
        Resend = 0xFE, // TODO
        IdentifyDevice = 0xF2 // TODO
    }

    pub mod keyboard {
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum KeyboardCommand {
            /// Scan set 3 only
            SetAllKeysToRepeatingOnly = 0xF7, // TODO
            /// Scan set 3 only
            SetAllKeysToMakeReleaseOnly = 0xF8, // TODO
            /// Scan set 3 only
            SetAllKeysToMakeOnly = 0xF9, // TODO
            /// Scan set 3 only
            SetAllKeysToRepeatingMakeRelease = 0xFa // TODO
        }

        /// Represents a PS2 keyboard command without a return and with additional data
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum KeyboardDataCommand {
            SetLeds = 0xED, // TODO
            SetTypematicOptions = 0xF3,  // TODO
            /// Scan set 3 only
            SetKeyRepeatingOnly = 0xFB, // TODO,
            /// Scan set 3 only
            SetKeyMakeReleaseOnly = 0xFC, // TODO
            /// Scan set 3 only
            SetKeyMakeOnly = 0xFD // TODO
        }

        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum KeyboardReturnCommand {
            Echo = 0xEE, // TODO
        }

        // This is in its own const since it's so weird
        // It *can* return depending on the data sent, but otherwise it doesn't
        const SET_GET_SCANCODE: u8 = 0xF0;
    }

    pub mod mouse {
        /// Represents a PS2 mouse command without a return and without additional data
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum MouseCommand {
            SetRemoteMode = 0xF0, // TODO
            SetWrapMode = 0xEE, // TODO
            ResetWrapMode = 0xEC, // TODO
            SetStreamMode = 0xEA, // TODO
            StatusRequest = 0xE9 // TODO // TODO is this returning?
        }

        /// Represents a PS2 mouse command with additional data
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum MouseCommandData {
            SetSampleRate = 0xF3, // TODO
            SetResolution = 0xE8 // TODO
        }
    }
}