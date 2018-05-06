use super::Keycode;

macro_rules! codes {
    {$($name:ident = $row:expr, $col:expr;)*} => {
        use super::Keycode;

        $(
            pub const $name: Keycode = ($col & 0x1F) | ($row & 0x7) << 5;
        )*
    }
}

#[allow(dead_code)] // Dead keys for completeness
pub mod codes {
    //! # Codes
    //!
    //! This module contains a list of US QWERTY key code constants, based around rows/columns on a keyboard.
    //! This is used because, for example, in a game using WASD, you're looking for the characters in that position, not those characters specifically.
    //! All non-character codes can represent the same key on any keyboard layout.
    
    codes! {
        ESCAPE = 0, 0;

        F1 = 0, 1;
        F2 = 0, 2;
        F3 = 0, 3;
        F4 = 0, 4;
        F5 = 0, 5;
        F6 = 0, 6;
        F7 = 0, 7;
        F8 = 0, 8;
        F9 = 0, 0;
        F10 = 0, 10;
        F11 = 0, 11;
        F12 = 0, 12;

        PRINT_SCREEN = 0, 13;
        SCROLL_LOCK = 0, 14;
        PAUSE = 0, 15;

        BACK_TICK = 1, 0;
        KEY_1 = 1, 1;
        KEY_2 = 1, 2;
        KEY_3 = 1, 3;
        KEY_4 = 1, 4;
        KEY_5 = 1, 5;
        KEY_6 = 1, 6;
        KEY_7 = 1, 7;
        KEY_8 = 1, 8;
        KEY_9 = 1, 9;
        KEY_0 = 1, 10;
        MINUS = 1, 11;
        EQUALS = 1, 12;
        BACKSPACE = 1, 13;

        INSERT = 1, 14;
        HOME = 1, 15;
        PAGE_UP = 1, 16;

        NUM_LOCK = 1, 17;
        NUM_PAD_FORWARD_SLASH = 1, 18;
        NUM_PAD_ASTERISK = 1, 19;
        NUM_PAD_MINUS = 1, 20;

        TAB = 2, 0;
        Q = 2, 1;
        W = 2, 2;
        E = 2, 3;
        R = 2, 4;
        T = 2, 5;
        Y = 2, 6;
        U = 2, 7;
        I = 2, 8;
        O = 2, 9;
        P = 2, 10;
        SQUARE_BRACKET_OPEN = 2, 11;
        SQUARE_BRACKET_CLOSE = 2, 12;
        BACK_SLASH = 2, 13;

        DELETE = 2, 14;
        END = 2, 15;
        PAGE_DOWN = 2, 16;

        NUM_PAD_7 = 2, 17;
        NUM_PAD_8 = 2, 18;
        NUM_PAD_9 = 2, 19;

        CAPS_LOCK = 3, 0;
        A = 3, 1;
        S = 3, 2;
        D = 3, 3;
        F = 3, 4;
        G = 3, 5;
        H = 3, 6;
        J = 3, 7;
        K = 3, 8;
        L = 3, 9;
        SEMI_COLON = 3, 10;
        SINGLE_QUOTE = 3, 11;
        ENTER = 3, 12;

        NUM_PAD_4 = 3, 13;
        NUM_PAD_5 = 3, 14;
        NUM_PAD_6 = 3, 15;
        NUM_PAD_PLUS = 3, 16;

        LEFT_SHIFT = 4, 0;
        Z = 4, 1;
        X = 4, 2;
        C = 4, 3;
        V = 4, 4;
        B = 4, 5;
        N = 4, 6;
        M = 4, 7;
        COMMA = 4, 8;
        PERIOD = 4, 9;
        FORWARD_SLASH = 4, 10;
        RIGHT_SHIFT = 4, 11;

        UP_ARROW = 4, 12;

        NUM_PAD_1 = 4, 13;
        NUM_PAD_2 = 4, 14;
        NUM_PAD_3 = 4, 15;

        LEFT_CONTROL = 5, 0;
        LEFT_WIN = 5, 1;
        LEFT_ALT = 5, 2;
        SPACE = 5, 3;
        RIGHT_ALT = 5, 4;
        RIGHT_WIN = 5, 5;
        FUNCTION = 5, 6;
        RIGHT_CONTROL = 5, 7;
        LEFT_ARROW = 5, 8;
        DOWN_ARROW = 5, 9;
        RIGHT_ARROW = 5, 10;
        NUM_PAD_0 = 5, 11;
        NUM_PAD_DELETE = 5, 12;
        NUM_PAD_ENTER = 5, 13;
    }
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
