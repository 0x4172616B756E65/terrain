use bevy::app::{Plugin, Startup};

use crate::player::inventory::{hud::load_hud, items::Item};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(Startup, load_hud);
    }
}

pub struct Inventory {
    pub active: Vec<Item>

}
