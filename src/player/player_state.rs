use std::collections::HashMap;

use crate::player::player_state::PlayerAction::*;
use bevy::input::{keyboard::KeyCode::{self, *}, mouse::MouseButton};

#[derive(Debug)]
pub struct PlayerState {
    pub fov: f32,
    pub keymap: HashMap<KeyCode, PlayerAction>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { 
            fov: 100., 
            keymap: [
                (KeyH, MOVE_FORWARDS),
                (KeyS, MOVE_LEFTWARDS),
                (KeyM, MOVE_BACKWARDS),
                (KeyT, MOVE_RIGHTWARDS),
                (ShiftLeft, MOVE_SPRINTING),

                (Space, FLY_UPWARDS),
                (Backspace, FLY_DOWNWARDS),

                (KeyE, INTERACT_GENERIC),
            ].iter().cloned().collect(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum PlayerAction {
    MOVE_FORWARDS,
    MOVE_BACKWARDS,
    MOVE_LEFTWARDS,
    MOVE_RIGHTWARDS,
    MOVE_SPRINTING,

    FLY_UPWARDS,
    FLY_DOWNWARDS,

    INTERACT_GENERIC,

    WEAPON_ATTACK,
    WEAPON_AIM,
    WEAPON_ACTION_1,
    WEAPON_ACTION_2,
    WEAPON_ACTION_3,
}
