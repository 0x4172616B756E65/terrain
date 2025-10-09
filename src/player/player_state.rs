use bevy::ecs::event::{Event, EventWriter};

use crate::player::player::Player;

pub struct PlayerState {
    pub is_pressing_movement_key: bool,
    pub debug_flying: bool,
    pub inventory_open: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { 
            is_pressing_movement_key: false, 
            debug_flying: false,
            inventory_open: true,
        }
    }
}

#[derive(Event)]
pub struct ToggleInventory/*(pub Player)*/;

