use std::collections::HashMap;

use crate::player::state::{player_state::PlayerAction::*, serde_keyboard::SerdeKeyCode, serde_mouse::SerdeMouseCode};
use bevy::input::{keyboard::KeyCode::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub fov: f32,
    pub keymap: HashMap<InputBinding, PlayerAction>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { 
            fov: 100., 
            keymap: [
                (InputBinding::Key(KeyH.into()), MoveForwards),
                (InputBinding::Key(KeyS.into()), MoveLeftwards),
                (InputBinding::Key(KeyT.into()), MoveRightwards),
                (InputBinding::Key(KeyM.into()), MoveBackwards),
                (InputBinding::Key(KeyA.into()), MoveSprinting),

                (InputBinding::Key(Space.into()), FlyUpwards),
                (InputBinding::Key(Backspace.into()), FlyDownwards),

                (InputBinding::Key(KeyW.into()), InteractGeneric),

                (InputBinding::MouseWheelUp, DebugIncreaseRenderDistance),
                (InputBinding::MouseWheelDown, DebugDecreaseRenderDistance)

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

    DebugIncreaseRenderDistance,
    DebugDecreaseRenderDistance,
}


