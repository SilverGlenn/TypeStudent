use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
    Thumb,
}

impl Finger {
    pub fn name(&self) -> &'static str {
        match self {
            Finger::LeftPinky => "Left Pinky",
            Finger::LeftRing => "Left Ring",
            Finger::LeftMiddle => "Left Middle",
            Finger::LeftIndex => "Left Index",
            Finger::RightIndex => "Right Index",
            Finger::RightMiddle => "Right Middle",
            Finger::RightRing => "Right Ring",
            Finger::RightPinky => "Right Pinky",
            Finger::Thumb => "Thumb",
        }
    }

    pub fn color_hex(&self) -> u32 {
        match self {
            Finger::LeftPinky => 0xE57373,  // Soft Red/Pink
            Finger::LeftRing => 0xFFB74D,   // Warm Orange
            Finger::LeftMiddle => 0x81C784, // Green
            Finger::LeftIndex => 0x64B5F6,  // Light Blue
            Finger::RightIndex => 0x4FC3F7, // Cyan/Sky Blue
            Finger::RightMiddle => 0x81C784,// Green
            Finger::RightRing => 0xFFB74D,  // Warm Orange
            Finger::RightPinky => 0xBA68C8, // Purple
            Finger::Thumb => 0xFFD54F,      // Gold/Amber
        }
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        let hex = self.color_hex();
        (
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }
}

pub fn get_finger_for_char(c: char) -> Finger {
    let lower = c.to_ascii_lowercase();
    match lower {
        '`' | '~' | '1' | '!' | 'q' | 'a' | 'z' => Finger::LeftPinky,
        '2' | '@' | 'w' | 's' | 'x' => Finger::LeftRing,
        '3' | '#' | 'e' | 'd' | 'c' => Finger::LeftMiddle,
        '4' | '$' | '5' | '%' | 'r' | 't' | 'f' | 'g' | 'v' | 'b' => Finger::LeftIndex,
        
        '6' | '^' | '7' | '&' | 'y' | 'u' | 'h' | 'j' | 'n' | 'm' => Finger::RightIndex,
        '8' | '*' | 'i' | 'k' | ',' | '<' => Finger::RightMiddle,
        '9' | '(' | 'o' | 'l' | '.' | '>' => Finger::RightRing,
        '0' | ')' | '-' | '_' | '=' | '+' | 'p' | '[' | '{' | ']' | '}' | '\\' | '|' | ';' | ':' | '\'' | '"' | '/' | '?' => Finger::RightPinky,
        
        ' ' => Finger::Thumb,
        _ => Finger::Thumb,
    }
}

#[derive(Clone, Debug)]
pub struct KeyDef {
    pub label: &'static str,
    pub shift_label: Option<&'static str>,
    pub char_val: char,
    pub finger: Finger,
    pub width_units: f32, // 1.0 = standard square key
}

pub fn get_keyboard_layout() -> Vec<Vec<KeyDef>> {
    vec![
        // Row 1: Numbers
        vec![
            KeyDef { label: "`", shift_label: Some("~"), char_val: '`', finger: Finger::LeftPinky, width_units: 1.0 },
            KeyDef { label: "1", shift_label: Some("!"), char_val: '1', finger: Finger::LeftPinky, width_units: 1.0 },
            KeyDef { label: "2", shift_label: Some("@"), char_val: '2', finger: Finger::LeftRing, width_units: 1.0 },
            KeyDef { label: "3", shift_label: Some("#"), char_val: '3', finger: Finger::LeftMiddle, width_units: 1.0 },
            KeyDef { label: "4", shift_label: Some("$"), char_val: '4', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "5", shift_label: Some("%"), char_val: '5', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "6", shift_label: Some("^"), char_val: '6', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "7", shift_label: Some("&"), char_val: '7', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "8", shift_label: Some("*"), char_val: '8', finger: Finger::RightMiddle, width_units: 1.0 },
            KeyDef { label: "9", shift_label: Some("("), char_val: '9', finger: Finger::RightRing, width_units: 1.0 },
            KeyDef { label: "0", shift_label: Some(")"), char_val: '0', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "-", shift_label: Some("_"), char_val: '-', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "=", shift_label: Some("+"), char_val: '=', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "Bksp", shift_label: None, char_val: '\x08', finger: Finger::RightPinky, width_units: 1.8 },
        ],
        // Row 2: QWERTY
        vec![
            KeyDef { label: "Tab", shift_label: None, char_val: '\t', finger: Finger::LeftPinky, width_units: 1.5 },
            KeyDef { label: "Q", shift_label: None, char_val: 'q', finger: Finger::LeftPinky, width_units: 1.0 },
            KeyDef { label: "W", shift_label: None, char_val: 'w', finger: Finger::LeftRing, width_units: 1.0 },
            KeyDef { label: "E", shift_label: None, char_val: 'e', finger: Finger::LeftMiddle, width_units: 1.0 },
            KeyDef { label: "R", shift_label: None, char_val: 'r', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "T", shift_label: None, char_val: 't', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "Y", shift_label: None, char_val: 'y', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "U", shift_label: None, char_val: 'u', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "I", shift_label: None, char_val: 'i', finger: Finger::RightMiddle, width_units: 1.0 },
            KeyDef { label: "O", shift_label: None, char_val: 'o', finger: Finger::RightRing, width_units: 1.0 },
            KeyDef { label: "P", shift_label: None, char_val: 'p', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "[", shift_label: Some("{"), char_val: '[', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "]", shift_label: Some("}"), char_val: ']', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "\\", shift_label: Some("|"), char_val: '\\', finger: Finger::RightPinky, width_units: 1.3 },
        ],
        // Row 3: ASDF (Home Row)
        vec![
            KeyDef { label: "Caps", shift_label: None, char_val: '\0', finger: Finger::LeftPinky, width_units: 1.8 },
            KeyDef { label: "A", shift_label: None, char_val: 'a', finger: Finger::LeftPinky, width_units: 1.0 },
            KeyDef { label: "S", shift_label: None, char_val: 's', finger: Finger::LeftRing, width_units: 1.0 },
            KeyDef { label: "D", shift_label: None, char_val: 'd', finger: Finger::LeftMiddle, width_units: 1.0 },
            KeyDef { label: "F", shift_label: None, char_val: 'f', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "G", shift_label: None, char_val: 'g', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "H", shift_label: None, char_val: 'h', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "J", shift_label: None, char_val: 'j', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "K", shift_label: None, char_val: 'k', finger: Finger::RightMiddle, width_units: 1.0 },
            KeyDef { label: "L", shift_label: None, char_val: 'l', finger: Finger::RightRing, width_units: 1.0 },
            KeyDef { label: ";", shift_label: Some(":"), char_val: ';', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "'", shift_label: Some("\""), char_val: '\'', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "Enter", shift_label: None, char_val: '\n', finger: Finger::RightPinky, width_units: 2.0 },
        ],
        // Row 4: ZXCV
        vec![
            KeyDef { label: "Shift", shift_label: None, char_val: '\0', finger: Finger::LeftPinky, width_units: 2.3 },
            KeyDef { label: "Z", shift_label: None, char_val: 'z', finger: Finger::LeftPinky, width_units: 1.0 },
            KeyDef { label: "X", shift_label: None, char_val: 'x', finger: Finger::LeftRing, width_units: 1.0 },
            KeyDef { label: "C", shift_label: None, char_val: 'c', finger: Finger::LeftMiddle, width_units: 1.0 },
            KeyDef { label: "V", shift_label: None, char_val: 'v', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "B", shift_label: None, char_val: 'b', finger: Finger::LeftIndex, width_units: 1.0 },
            KeyDef { label: "N", shift_label: None, char_val: 'n', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: "M", shift_label: None, char_val: 'm', finger: Finger::RightIndex, width_units: 1.0 },
            KeyDef { label: ",", shift_label: Some("<"), char_val: ',', finger: Finger::RightMiddle, width_units: 1.0 },
            KeyDef { label: ".", shift_label: Some(">"), char_val: '.', finger: Finger::RightRing, width_units: 1.0 },
            KeyDef { label: "/", shift_label: Some("?"), char_val: '/', finger: Finger::RightPinky, width_units: 1.0 },
            KeyDef { label: "Shift", shift_label: None, char_val: '\0', finger: Finger::RightPinky, width_units: 2.5 },
        ],
        // Row 5: Spacebar
        vec![
            KeyDef { label: "Ctrl", shift_label: None, char_val: '\0', finger: Finger::LeftPinky, width_units: 1.5 },
            KeyDef { label: "Alt", shift_label: None, char_val: '\0', finger: Finger::LeftPinky, width_units: 1.2 },
            KeyDef { label: "Space", shift_label: None, char_val: ' ', finger: Finger::Thumb, width_units: 6.2 },
            KeyDef { label: "Alt", shift_label: None, char_val: '\0', finger: Finger::RightPinky, width_units: 1.2 },
            KeyDef { label: "Ctrl", shift_label: None, char_val: '\0', finger: Finger::RightPinky, width_units: 1.5 },
        ],
    ]
}
