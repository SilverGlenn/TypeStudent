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

const BASE_LEFT_PINKY: HandFingerState = HandFingerState { finger: Finger::LeftPinky, is_active: false, label: "Pinky", home_key: "A", normal_height: 55.0, active_height: 72.0, width: 34.0 };
const BASE_LEFT_RING: HandFingerState = HandFingerState { finger: Finger::LeftRing, is_active: false, label: "Ring", home_key: "S", normal_height: 75.0, active_height: 92.0, width: 34.0 };
const BASE_LEFT_MIDDLE: HandFingerState = HandFingerState { finger: Finger::LeftMiddle, is_active: false, label: "Middle", home_key: "D", normal_height: 90.0, active_height: 108.0, width: 36.0 };
const BASE_LEFT_INDEX: HandFingerState = HandFingerState { finger: Finger::LeftIndex, is_active: false, label: "Index", home_key: "F", normal_height: 80.0, active_height: 98.0, width: 36.0 };
const BASE_LEFT_THUMB: HandFingerState = HandFingerState { finger: Finger::Thumb, is_active: false, label: "Thumb", home_key: "Space", normal_height: 50.0, active_height: 68.0, width: 40.0 };
const BASE_RIGHT_THUMB: HandFingerState = HandFingerState { finger: Finger::Thumb, is_active: false, label: "Thumb", home_key: "Space", normal_height: 50.0, active_height: 68.0, width: 40.0 };
const BASE_RIGHT_INDEX: HandFingerState = HandFingerState { finger: Finger::RightIndex, is_active: false, label: "Index", home_key: "J", normal_height: 80.0, active_height: 98.0, width: 36.0 };
const BASE_RIGHT_MIDDLE: HandFingerState = HandFingerState { finger: Finger::RightMiddle, is_active: false, label: "Middle", home_key: "K", normal_height: 90.0, active_height: 108.0, width: 36.0 };
const BASE_RIGHT_RING: HandFingerState = HandFingerState { finger: Finger::RightRing, is_active: false, label: "Ring", home_key: "L", normal_height: 75.0, active_height: 92.0, width: 34.0 };
const BASE_RIGHT_PINKY: HandFingerState = HandFingerState { finger: Finger::RightPinky, is_active: false, label: "Pinky", home_key: ";", normal_height: 55.0, active_height: 72.0, width: 34.0 };

impl HandsGuideModel {
    pub fn for_active_target(active: Option<Finger>, target_char: Option<char>) -> Self {
        let mut model = Self {
            active_finger: active,
            target_char,
            left_pinky: BASE_LEFT_PINKY,
            left_ring: BASE_LEFT_RING,
            left_middle: BASE_LEFT_MIDDLE,
            left_index: BASE_LEFT_INDEX,
            left_thumb: BASE_LEFT_THUMB,
            right_thumb: BASE_RIGHT_THUMB,
            right_index: BASE_RIGHT_INDEX,
            right_middle: BASE_RIGHT_MIDDLE,
            right_ring: BASE_RIGHT_RING,
            right_pinky: BASE_RIGHT_PINKY,
        };

        if let Some(finger) = active {
            match finger {
                Finger::LeftPinky => model.left_pinky.is_active = true,
                Finger::LeftRing => model.left_ring.is_active = true,
                Finger::LeftMiddle => model.left_middle.is_active = true,
                Finger::LeftIndex => model.left_index.is_active = true,
                Finger::Thumb => {
                    model.left_thumb.is_active = true;
                    model.right_thumb.is_active = true;
                }
                Finger::RightIndex => model.right_index.is_active = true,
                Finger::RightMiddle => model.right_middle.is_active = true,
                Finger::RightRing => model.right_ring.is_active = true,
                Finger::RightPinky => model.right_pinky.is_active = true,
            }
        }

        model
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
