use drivers::keyboard::Keycode;

#[allow(dead_code)] // Dead keys for completeness
pub mod codes {
    //! # Codes
    //!
    //! This module contains a list of US QWERTY key code constants.

    use drivers::keyboard::Keycode;

    pub const F1: Keycode = 0x00;
    pub const F2: Keycode = 0x01;
    pub const F3: Keycode = 0x02;
    pub const F4: Keycode = 0x03;
    pub const F5: Keycode = 0x04;
    pub const F6: Keycode = 0x05;
    pub const F7: Keycode = 0x06;
    pub const F8: Keycode = 0x07;
    pub const F9: Keycode = 0x08;
    pub const F10: Keycode = 0x09;
    pub const F11: Keycode = 0x0A;
    pub const F12: Keycode = 0x0B;

    pub const KEY_1: Keycode = 0x0C;
    pub const KEY_2: Keycode = 0x0D;
    pub const KEY_3: Keycode = 0x0E;
    pub const KEY_4: Keycode = 0x0F;
    pub const KEY_5: Keycode = 0x10;
    pub const KEY_6: Keycode = 0x11;
    pub const KEY_7: Keycode = 0x12;
    pub const KEY_8: Keycode = 0x13;
    pub const KEY_9: Keycode = 0x14;
    pub const KEY_0: Keycode = 0x15;

    pub const Q: Keycode = 0x16;
    pub const W: Keycode = 0x17;
    pub const E: Keycode = 0x18;
    pub const R: Keycode = 0x19;
    pub const T: Keycode = 0x1A;
    pub const Y: Keycode = 0x1B;
    pub const U: Keycode = 0x1C;
    pub const I: Keycode = 0x1D;
    pub const O: Keycode = 0x1E;
    pub const P: Keycode = 0x1F;
    pub const A: Keycode = 0x20;
    pub const S: Keycode = 0x21;
    pub const D: Keycode = 0x22;
    pub const F: Keycode = 0x23;
    pub const G: Keycode = 0x24;
    pub const H: Keycode = 0x25;
    pub const J: Keycode = 0x26;
    pub const K: Keycode = 0x27;
    pub const L: Keycode = 0x28;
    pub const Z: Keycode = 0x29;
    pub const X: Keycode = 0x2A;
    pub const C: Keycode = 0x2B;
    pub const V: Keycode = 0x2C;
    pub const B: Keycode = 0x2D;
    pub const N: Keycode = 0x2E;
    pub const M: Keycode = 0x2F;

    pub const SPACE: Keycode = 0x30;
    pub const EQUALS: Keycode = 0x31;
    pub const MINUS: Keycode = 0x32;
    pub const COMMA: Keycode = 0x33;
    pub const PERIOD: Keycode = 0x34;
    pub const SEMI_COLON: Keycode = 0x35;
    pub const SINGLE_QUOTE: Keycode = 0x36;
    pub const BACK_TICK: Keycode = 0x37;
    pub const SQUARE_BRACKET_OPEN: Keycode = 0x38;
    pub const SQUARE_BRACKET_CLOSE: Keycode = 0x39;
    pub const FORWARD_SLASH: Keycode = 0x3A;
    pub const BACK_SLASH: Keycode = 0x3B;
    pub const ESCAPE: Keycode = 0x3C;
    pub const ENTER: Keycode = 0x3D;
    pub const BACKSPACE: Keycode = 0x3E;
    pub const TAB: Keycode = 0x3F;

    pub const PRINT_SCREEN: Keycode = 0x40;
    pub const PAUSE: Keycode = 0x41;
    pub const INSERT: Keycode = 0x42;
    pub const DELETE: Keycode = 0x43;
    pub const HOME: Keycode = 0x44;
    pub const PAGE_UP: Keycode = 0x45;
    pub const PAGE_DOWN: Keycode = 0x46;
    pub const END: Keycode = 0x47;

    pub const FUNCTION: Keycode = 0x48;
    pub const LEFT_CONTROL: Keycode = 0x49;
    pub const RIGHT_CONTROL: Keycode = 0x4A;
    pub const LEFT_SHIFT: Keycode = 0x4B;
    pub const RIGHT_SHIFT: Keycode = 0x4C;
    pub const LEFT_WIN: Keycode = 0x4D;
    pub const RIGHT_WIN: Keycode = 0x4E;
    pub const LEFT_ALT: Keycode = 0x4F;
    pub const RIGHT_ALT: Keycode = 0x50;

    pub const SCROLL_LOCK: Keycode = 0x51;
    pub const NUM_LOCK: Keycode = 0x52;
    pub const CAPS_LOCK: Keycode = 0x53;
    pub const UP_ARROW: Keycode = 0x54;
    pub const LEFT_ARROW: Keycode = 0x55;
    pub const DOWN_ARROW: Keycode = 0x56;
    pub const RIGHT_ARROW: Keycode = 0x57;

    pub const NUM_PAD_0: Keycode = 0x58;
    pub const NUM_PAD_1: Keycode = 0x59;
    pub const NUM_PAD_2: Keycode = 0x5A;
    pub const NUM_PAD_3: Keycode = 0x5B;
    pub const NUM_PAD_4: Keycode = 0x5C;
    pub const NUM_PAD_5: Keycode = 0x5D;
    pub const NUM_PAD_6: Keycode = 0x5E;
    pub const NUM_PAD_7: Keycode = 0x5F;
    pub const NUM_PAD_8: Keycode = 0x60;
    pub const NUM_PAD_9: Keycode = 0x61;
    pub const NUM_PAD_PLUS: Keycode = 0x62;
    pub const NUM_PAD_MINUS: Keycode = 0x63;
    pub const NUM_PAD_ENTER: Keycode = 0x64;
    pub const NUM_PAD_DELETE: Keycode = 0x65;
    pub const NUM_PAD_FORWARD_SLASH: Keycode = 0x66;
    pub const NUM_PAD_ASTERISK: Keycode = 0x67;
}

/// Gets the US QWERTY character(s) for the given Flower keycode. The first element represents the lower-case, and the second the upper.
pub fn get_us_qwerty_char(keycode: Keycode) -> Option<(char, char)> {
    match keycode {
        codes::KEY_1 => Some(('1', '!')),
        codes::KEY_2 => Some(('2', '@')),
        codes::KEY_3 => Some(('3', '#')),
        codes::KEY_4 => Some(('4', '$')),
        codes::KEY_5 => Some(('5', '%')),
        codes::KEY_6 => Some(('6', '^')),
        codes::KEY_7 => Some(('7', '&')),
        codes::KEY_8 => Some(('8', '*')),
        codes::KEY_9 => Some(('9', '(')),
        codes::KEY_0 => Some(('0', ')')),
        codes::MINUS => Some(('-', '_')),
        codes::EQUALS => Some(('=', '+')),
        codes::BACKSPACE => Some(('\x08', '\x08')),
        codes::TAB => Some(('\t', '\t')),
        codes::Q => Some(('q', 'Q')),
        codes::W => Some(('w', 'W')),
        codes::E => Some(('e', 'E')),
        codes::R => Some(('r', 'R')),
        codes::T => Some(('t', 'T')),
        codes::Y => Some(('y', 'Y')),
        codes::U => Some(('u', 'U')),
        codes::I => Some(('i', 'I')),
        codes::O => Some(('o', 'O')),
        codes::P => Some(('p', 'P')),
        codes::SQUARE_BRACKET_OPEN => Some(('[', '{')),
        codes::SQUARE_BRACKET_CLOSE => Some((']', '}')),
        codes::ENTER => Some(('\n', '\n')),
        codes::A => Some(('a', 'A')),
        codes::S => Some(('s', 'S')),
        codes::D => Some(('d', 'D')),
        codes::F => Some(('f', 'F')),
        codes::G => Some(('g', 'G')),
        codes::H => Some(('h', 'H')),
        codes::J => Some(('j', 'J')),
        codes::K => Some(('k', 'K')),
        codes::L => Some(('l', 'L')),
        codes::SEMI_COLON => Some((';', ':')),
        codes::SINGLE_QUOTE => Some(('\'', '\"')),
        codes::BACK_TICK => Some(('`', '~')),
        codes::BACK_SLASH => Some(('\\', '|')),
        codes::Z => Some(('z', 'Z')),
        codes::X => Some(('x', 'X')),
        codes::C => Some(('c', 'C')),
        codes::V => Some(('v', 'V')),
        codes::B => Some(('b', 'B')),
        codes::N => Some(('n', 'N')),
        codes::M => Some(('m', 'M')),
        codes::COMMA => Some((',', '<')),
        codes::PERIOD => Some(('.', '>')),
        codes::FORWARD_SLASH => Some(('/', '?')),
        codes::SPACE => Some((' ', ' ')),
        _ => None,
    }
}

/// Gets the Flower keycode for the given PS/2 scanset 2 scancode
pub fn get_code_ps2_set_2(scancode: u8) -> Option<Keycode> {
    use self::codes::*;

    match scancode {
        0x01 => Some(F9),
        0x03 => Some(F5),
        0x04 => Some(F3),
        0x05 => Some(F1),
        0x06 => Some(F2),
        0x07 => Some(F12),
        0x09 => Some(F10),
        0x0A => Some(F8),
        0x0B => Some(F6),
        0x0C => Some(F4),
        0x0D => Some(TAB),
        0x0E => Some(BACK_TICK),
        0x11 => Some(LEFT_ALT),
        0x12 => Some(LEFT_SHIFT),
        0x14 => Some(LEFT_CONTROL),
        0x15 => Some(Q),
        0x16 => Some(KEY_1),
        0x1A => Some(Z),
        0x1B => Some(S),
        0x1C => Some(A),
        0x1D => Some(W),
        0x1E => Some(KEY_2),
        0x21 => Some(C),
        0x22 => Some(X),
        0x23 => Some(D),
        0x24 => Some(E),
        0x25 => Some(KEY_4),
        0x26 => Some(KEY_3),
        0x29 => Some(SPACE),
        0x2A => Some(V),
        0x2B => Some(F),
        0x2C => Some(T),
        0x2D => Some(R),
        0x2E => Some(KEY_5),
        0x31 => Some(N),
        0x32 => Some(B),
        0x33 => Some(H),
        0x34 => Some(G),
        0x35 => Some(Y),
        0x36 => Some(KEY_6),
        0x3A => Some(M),
        0x3B => Some(J),
        0x3C => Some(U),
        0x3D => Some(KEY_7),
        0x3E => Some(KEY_8),
        0x41 => Some(COMMA),
        0x42 => Some(K),
        0x43 => Some(I),
        0x44 => Some(O),
        0x45 => Some(KEY_0),
        0x46 => Some(KEY_9),
        0x49 => Some(PERIOD),
        0x4A => Some(FORWARD_SLASH),
        0x4B => Some(L),
        0x4C => Some(SEMI_COLON),
        0x4D => Some(P),
        0x4E => Some(MINUS),
        0x52 => Some(SINGLE_QUOTE),
        0x54 => Some(SQUARE_BRACKET_OPEN),
        0x55 => Some(EQUALS),
        0x58 => Some(CAPS_LOCK),
        0x59 => Some(RIGHT_SHIFT),
        0x5A => Some(ENTER),
        0x5B => Some(SQUARE_BRACKET_CLOSE),
        0x5D => Some(BACK_SLASH),
        0x66 => Some(BACKSPACE),
        0x69 => Some(NUM_PAD_1),
        0x6B => Some(NUM_PAD_4),
        0x6C => Some(NUM_PAD_7),
        0x70 => Some(NUM_PAD_0),
        0x71 => Some(NUM_PAD_DELETE),
        0x72 => Some(NUM_PAD_2),
        0x73 => Some(NUM_PAD_5),
        0x74 => Some(NUM_PAD_6),
        0x75 => Some(NUM_PAD_8),
        0x76 => Some(ESCAPE),
        0x77 => Some(NUM_LOCK),
        0x78 => Some(F11),
        0x79 => Some(NUM_PAD_PLUS),
        0x7A => Some(NUM_PAD_3),
        0x7B => Some(NUM_PAD_MINUS),
        0x7C => Some(NUM_PAD_ASTERISK),
        0x7D => Some(NUM_PAD_9),
        0x7E => Some(SCROLL_LOCK),
        0x83 => Some(F7),
        _ => None,
    }
}

/// Gets the Flower keycode for the given PS/2 extended scanset 2 scancode
pub fn get_extended_code_ps2_set_2(extended_code: u8) -> Option<Keycode> {
    use self::codes::*;

    match extended_code {
        0x11 => Some(RIGHT_ALT),
        0x14 => Some(RIGHT_CONTROL),
        0x4A => Some(NUM_PAD_FORWARD_SLASH),
        0x5A => Some(NUM_PAD_ENTER),
        0x69 => Some(END),
        0x6C => Some(HOME),
        0x70 => Some(INSERT),
        0x71 => Some(DELETE),
        0x7A => Some(PAGE_DOWN),
        0x7D => Some(PAGE_UP),
        _ => None,
    }
}
