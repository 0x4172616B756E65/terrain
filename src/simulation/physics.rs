use bevy::prelude::*;
use bevy::{app::{Plugin, Update}, ecs::system::{Query, Res}, time::Time};

use crate::player::player_attack::DebugShootEvent;
use crate::simulation::ballistics::ammunition::{Ballistics, Bullet};

pub const GRAVITY: f32 = 9.81;
const AIR_PRESSURE: f32 = 101325_f32;
const AIR_CONSTANT: f32 = 287.05;

pub struct BallisticsPlugin;

impl Plugin for BallisticsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_event::<DebugShootEvent>()
            .add_systems(Update, update_projectiles);
    }
}

#[derive(Resource)]
pub struct WorldState {
    temperature: f32, 
}

impl Default for WorldState {
    fn default() -> Self {
        WorldState { temperature: 20. }
    }
}

impl WorldState {
    pub fn new() -> Self {
        WorldState { 
            temperature: 20_f32
        }
    }

    pub fn get_air_density(&self) -> f32 {
        AIR_PRESSURE / (AIR_CONSTANT * (273.15 + &self.temperature))
    }
}

fn update_projectiles(
    time: Res<Time>,
    mut projectiles: Query<(&mut Bullet, &mut Transform)>
) {
    let delta_time = time.delta_secs();

    for (mut ballistics, mut transform) in projectiles.iter_mut() {
        ballistics.step(delta_time); 
        transform.translation = ballistics.position;
    }
}
