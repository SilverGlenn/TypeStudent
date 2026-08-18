use crate::components::keyboard::Finger;

#[derive(Clone, Copy, Debug)]
pub struct HandFingerState {
    pub is_active: bool,
    pub label: &'static str,
    pub home_key: &'static str,
}

pub struct HandsGuideModel {
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
    pub fn for_active_finger(active: Option<Finger>) -> Self {
        Self {
            left_pinky: HandFingerState {
                is_active: active == Some(Finger::LeftPinky),
                label: "Pinky",
                home_key: "A",
            },
            left_ring: HandFingerState {
                is_active: active == Some(Finger::LeftRing),
                label: "Ring",
                home_key: "S",
            },
            left_middle: HandFingerState {
                is_active: active == Some(Finger::LeftMiddle),
                label: "Middle",
                home_key: "D",
            },
            left_index: HandFingerState {
                is_active: active == Some(Finger::LeftIndex),
                label: "Index",
                home_key: "F",
            },
            left_thumb: HandFingerState {
                is_active: active == Some(Finger::Thumb),
                label: "Thumb",
                home_key: "Space",
            },
            right_thumb: HandFingerState {
                is_active: active == Some(Finger::Thumb),
                label: "Thumb",
                home_key: "Space",
            },
            right_index: HandFingerState {
                is_active: active == Some(Finger::RightIndex),
                label: "Index",
                home_key: "J",
            },
            right_middle: HandFingerState {
                is_active: active == Some(Finger::RightMiddle),
                label: "Middle",
                home_key: "K",
            },
            right_ring: HandFingerState {
                is_active: active == Some(Finger::RightRing),
                label: "Ring",
                home_key: "L",
            },
            right_pinky: HandFingerState {
                is_active: active == Some(Finger::RightPinky),
                label: "Pinky",
                home_key: ";",
            },
        }
    }
}
