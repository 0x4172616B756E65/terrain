use std::collections::HashMap;

use crate::player::config::{player_config::PlayerAction::*, serde_keyboard::SerdeKeyCode, serde_mouse::SerdeMouseCode};
use bevy::input::{keyboard::KeyCode::*, mouse::MouseButton::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub fov: f32,
    pub lod_radius: u32,
    pub render_radius: u32,
    pub keymap: HashMap<InputBinding, PlayerAction>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig { 
            fov: 100., 
            lod_radius: 1,
            render_radius: 64,
            keymap: [
                (InputBinding::Key(KeyH.into()), MoveForwards),
                (InputBinding::Key(KeyS.into()), MoveLeftwards),
                (InputBinding::Key(KeyT.into()), MoveRightwards),
                (InputBinding::Key(KeyM.into()), MoveBackwards),
                (InputBinding::Key(KeyA.into()), MoveSprinting),

                (InputBinding::Key(Space.into()), FlyUpwards),
                (InputBinding::Key(Backspace.into()), FlyDownwards),

                (InputBinding::Key(KeyW.into()), InteractGeneric),

                (InputBinding::MouseButton(Left.into()), DebugShootBullet),
                (InputBinding::MouseWheelUp, DebugIncreaseRenderDistance),
                (InputBinding::MouseWheelDown, DebugDecreaseRenderDistance),
                (InputBinding::Key(KeyX.into()), DebugToggleFlight),

            ].iter().cloned().collect(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputBinding {
    Key(SerdeKeyCode),
    MouseButton(SerdeMouseCode),
    MouseWheelUp,
    MouseWheelDown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerAction {
    MoveForwards,
    MoveBackwards,
    MoveLeftwards,
    MoveRightwards,
    MoveSprinting,

    FlyUpwards,
    FlyDownwards,

    InteractGeneric,

    WeaponAttack,
    WeaponAim,
    WeaponAction1,
    WeaponAction2,
    WeaponAction3,

    DebugShootBullet,
    DebugIncreaseRenderDistance,
    DebugDecreaseRenderDistance,
    DebugToggleFlight,
}


