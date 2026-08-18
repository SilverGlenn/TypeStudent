use crate::components::keyboard::Finger;

#[derive(Clone, Copy, Debug)]
pub struct HandFingerState {
    pub finger: Finger,
    pub is_active: bool,
    pub label: &'static str,
    pub home_key: &'static str,
    pub normal_height: f32,
    pub active_height: f32,
    pub width: f32,
}

pub struct HandsGuideModel {
    pub active_finger: Option<Finger>,
    pub target_char: Option<char>,
    pub left_pinky: HandFingerState,
    pub left_ring: HandFingerState,
    pub left_middle: HandFingerState,
    pub left_index: HandFingerState,
    pub left_thumb: HandFingerState,
    pub right_thumb: HandFingerState,
    pub right_index: HandFingerState,
    pub right_middle: HandFingerState,
    pub right_ring: HandFingerState,
    pub right_pinky: HandFingerState,
}

impl HandsGuideModel {
    pub fn for_active_target(active: Option<Finger>, target_char: Option<char>) -> Self {
        Self {
            active_finger: active,
            target_char,
            left_pinky: HandFingerState {
                finger: Finger::LeftPinky,
                is_active: active == Some(Finger::LeftPinky),
                label: "Pinky",
                home_key: "A",
                normal_height: 65.0,
                active_height: 82.0,
                width: 28.0,
            },
            left_ring: HandFingerState {
                finger: Finger::LeftRing,
                is_active: active == Some(Finger::LeftRing),
                label: "Ring",
                home_key: "S",
                normal_height: 85.0,
                active_height: 102.0,
                width: 28.0,
            },
            left_middle: HandFingerState {
                finger: Finger::LeftMiddle,
                is_active: active == Some(Finger::LeftMiddle),
                label: "Middle",
                home_key: "D",
                normal_height: 100.0,
                active_height: 118.0,
                width: 30.0,
            },
            left_index: HandFingerState {
                finger: Finger::LeftIndex,
                is_active: active == Some(Finger::LeftIndex),
                label: "Index",
                home_key: "F",
                normal_height: 90.0,
                active_height: 108.0,
                width: 30.0,
            },
            left_thumb: HandFingerState {
                finger: Finger::Thumb,
                is_active: active == Some(Finger::Thumb),
                label: "Thumb",
                home_key: "Space",
                normal_height: 52.0,
                active_height: 68.0,
                width: 32.0,
            },
            right_thumb: HandFingerState {
                finger: Finger::Thumb,
                is_active: active == Some(Finger::Thumb),
                label: "Thumb",
                home_key: "Space",
                normal_height: 52.0,
                active_height: 68.0,
                width: 32.0,
            },
            right_index: HandFingerState {
                finger: Finger::RightIndex,
                is_active: active == Some(Finger::RightIndex),
                label: "Index",
                home_key: "J",
                normal_height: 90.0,
                active_height: 108.0,
                width: 30.0,
            },
            right_middle: HandFingerState {
                finger: Finger::RightMiddle,
                is_active: active == Some(Finger::RightMiddle),
                label: "Middle",
                home_key: "K",
                normal_height: 100.0,
                active_height: 118.0,
                width: 30.0,
            },
            right_ring: HandFingerState {
                finger: Finger::RightRing,
                is_active: active == Some(Finger::RightRing),
                label: "Ring",
                home_key: "L",
                normal_height: 85.0,
                active_height: 102.0,
                width: 28.0,
            },
            right_pinky: HandFingerState {
                finger: Finger::RightPinky,
                is_active: active == Some(Finger::RightPinky),
                label: "Pinky",
                home_key: ";",
                normal_height: 65.0,
                active_height: 82.0,
                width: 28.0,
            },
        }
    }

    pub fn active_finger_instruction(&self) -> String {
        let target_display = match self.target_char {
            Some(' ') => "Spacebar".to_string(),
            Some(c) => format!("'{}'", c.to_uppercase()),
            None => "next key".to_string(),
        };

        match self.active_finger {
            Some(Finger::LeftPinky) => format!("Press {} with Left Pinky (Home: A)", target_display),
            Some(Finger::LeftRing) => format!("Press {} with Left Ring Finger (Home: S)", target_display),
            Some(Finger::LeftMiddle) => format!("Press {} with Left Middle Finger (Home: D)", target_display),
            Some(Finger::LeftIndex) => format!("Press {} with Left Index Finger (Home: F)", target_display),
            Some(Finger::Thumb) => format!("Press {} with Thumb (Resting on Spacebar)", target_display),
            Some(Finger::RightIndex) => format!("Press {} with Right Index Finger (Home: J)", target_display),
            Some(Finger::RightMiddle) => format!("Press {} with Right Middle Finger (Home: K)", target_display),
            Some(Finger::RightRing) => format!("Press {} with Right Ring Finger (Home: L)", target_display),
            Some(Finger::RightPinky) => format!("Press {} with Right Pinky (Home: ;)", target_display),
            None => "Place fingers on Home Row (ASDF - JKL;)".to_string(),
        }
    }
}
