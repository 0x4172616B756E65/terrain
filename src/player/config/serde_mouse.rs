use bevy::input::mouse::MouseButton;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerdeMouseCode(String);

impl From<MouseButton> for SerdeMouseCode {
    fn from(k: MouseButton) -> Self {
        match k {
            MouseButton::Other(_) => panic!("Other not supported"),
            _ => SerdeMouseCode(format!("{:?}", k)),
        }
    }
}

impl From<SerdeMouseCode> for MouseButton {
    fn from(k: SerdeMouseCode) -> Self {
        match k.0.as_str() {
            "MouseLeft" => MouseButton::Left,
            "MouseRight" => MouseButton::Right   ,
            "MouseForward" => MouseButton::Forward,
            "MouseMiddle" => MouseButton::Middle,
            "MouseBack" => MouseButton::Back,
            _ => panic!("Unknown key code string: {}", k.0),
        }
    }
}
