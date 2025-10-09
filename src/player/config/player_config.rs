use std::collections::HashMap;

use crate::player::config::{player_config::{PlayerAction::*, PressKind::*}, serde_keyboard::SerdeKeyCode, serde_mouse::SerdeMouseCode};
use bevy::input::{keyboard::KeyCode::*, mouse::MouseButton::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub fov: f32,
    pub lod_radius: u32,
    pub render_radius: u32,
    pub keymap: HashMap<InputBinding, (PlayerAction, PressKind)>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig { 
            fov: 100., 
            lod_radius: 1,
            render_radius: 64,
            keymap: [
                (InputBinding::Key(KeyH.into()), (MoveForwards, Momentary)),
                (InputBinding::Key(KeyS.into()), (MoveLeftwards, Momentary)),
                (InputBinding::Key(KeyT.into()), (MoveRightwards, Momentary)),
                (InputBinding::Key(KeyM.into()), (MoveBackwards, Momentary)),
                (InputBinding::Key(KeyA.into()), (MoveSprinting, Momentary)),

                (InputBinding::Key(Space.into()), (FlyUpwards, Momentary)),
                (InputBinding::Key(Backspace.into()), (FlyDownwards, Momentary)),

                (InputBinding::Key(KeyW.into()), (InteractGeneric, MonoStable)),
                (InputBinding::Key(Tab.into()), (OpenInventory, MonoStable)),

                (InputBinding::MouseButton(Left.into()), (DebugShootBullet, MonoStable)),
                (InputBinding::MouseWheelUp, (DebugIncreaseRenderDistance, MonoStable)),
                (InputBinding::MouseWheelDown, (DebugDecreaseRenderDistance, MonoStable)),
                (InputBinding::Key(KeyX.into()), (DebugToggleFlight, MonoStable)),

            ].iter().cloned().collect(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PressKind {
    MonoStable,
    Momentary
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputBinding {
    Key(SerdeKeyCode),
    MouseButton(SerdeMouseCode),
    MouseWheelUp,
    MouseWheelDown,
}

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum PlayerAction {
    MoveForwards,
    MoveBackwards,
    MoveLeftwards,
    MoveRightwards,
    MoveSprinting,

    FlyUpwards,
    FlyDownwards,

    InteractGeneric,
    OpenInventory,

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


