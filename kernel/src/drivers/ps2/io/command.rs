//! All PS/2 command related functionality or constants
// TODO: Document module and all command functionalities

/// Enums representing controller commands
pub mod controller {
    // TODO: Add remaining unimplemented controller commands
    /// Represents a PS/2 controller command
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum Command {
        DisablePort2 = 0xA7,
        EnablePort2 = 0xA8,
        DisablePort1 = 0xAD,
        EnablePort1 = 0xAE,
        WriteCommandPort2 = 0xD4,
        /// Returns
        ReadConfig = 0x20,
        /// Returns
        TestController = 0xAA,
        /// Returns
        TestPort1 = 0xAB,
        /// Returns
        TestPort2 = 0xA9,
    }

    /// Represents a PS/2 controller command with a data value
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum DataCommand {
        WriteConfig = 0x60,
    }
}

/// Enums representing PS/2 device commands
pub mod device {
    /// Represents a general PS/2 device command without additional data
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    pub enum Command {
        SetDefaults = 0xF6,
        Reset = 0xFF,
        /// Returns
        IdentifyDevice = 0xF2,
        /// Returns
        Echo = 0xEE,
        ResetEcho = 0xEC,
    }

    pub mod keyboard {
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum Command {
            /// Scan set 3 only
            SetAllKeysToRepeatingOnly = 0xF7, // TODO: Call
            /// Scan set 3 only
            SetAllKeysToMakeReleaseOnly = 0xF8, // TODO: Call
            /// Scan set 3 only
            SetAllKeysToMakeOnly = 0xF9, // TODO: Call
            /// Scan set 3 only
            SetAllKeysToRepeatingMakeRelease = 0xFA, // TODO: Call
        }

        /// Represents a PS/2 keyboard command where additional data can be sent
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum DataCommand {
            SetLeds = 0xED, // TODO: Call
            SetTypematicOptions = 0xF3,  // TODO: Call
            /// Scan set 3 only
            SetKeyRepeatingOnly = 0xFB, // TODO: Call
            /// Scan set 3 only
            SetKeyMakeReleaseOnly = 0xFC, // TODO: Call
            /// Scan set 3 only
            SetKeyMakeOnly = 0xFD, // TODO: Call
            SetGetScancode = 0xF0, // TODO: Call and document weirdness
        }
    }

    pub mod mouse {
        /// Represents a PS/2 mouse command without a return and without additional data
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum Command {
            SetRemoteMode = 0xF0, // TODO: Call
            SetWrapMode = 0xEE, // TODO: Call
            SetStreamMode = 0xEA, // TODO: Call
            StatusRequest = 0xE9, // TODO: Call
            RequestSinglePacket = 0xEB, // TODO: Call (check device removal; is this a returning packet?)
        }

        /// Represents a PS/2 mouse command where additional data can be sent
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        pub enum DataCommand {
            SetSampleRate = 0xF3, // TODO: Call
            SetResolution = 0xE8, // TODO: Call
        }
    }
}
