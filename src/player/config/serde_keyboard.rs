use bevy::input::keyboard::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerdeKeyCode(String);

impl From<KeyCode> for SerdeKeyCode {
    fn from(k: KeyCode) -> Self {
        match k {
            KeyCode::Unidentified(_) => panic!("Unidentified keys not supported"),
            _ => SerdeKeyCode(format!("{:?}", k)),
        }
    }
}

impl From<SerdeKeyCode> for KeyCode {
    fn from(k: SerdeKeyCode) -> Self {
        match k.0.as_str() {
            "KeyH" => KeyCode::KeyH,
            "KeyS" => KeyCode::KeyS,
            "KeyM" => KeyCode::KeyM,
            "KeyT" => KeyCode::KeyT,
            "KeyE" => KeyCode::KeyE,
            "ShiftLeft" => KeyCode::ShiftLeft,
            "Space" => KeyCode::Space,
            "Backspace" => KeyCode::Backspace,
            _ => panic!("Unknown key code string: {}", k.0),
        }
    }
}
