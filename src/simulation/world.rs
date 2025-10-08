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
            .add_systems(Update, (step_time, step_projectiles));
    }
}

#[derive(Resource)]
pub struct WorldState {
    time: f32,
    second_passed: bool,
    temperature: f32, 
}

impl Default for WorldState {
    fn default() -> Self {
        WorldState { 
            time: 0.00,
            second_passed: false,
            temperature: 20. 
        }
    }
}

impl WorldState {
    pub fn get_air_density(&self) -> f32 {
        AIR_PRESSURE / (AIR_CONSTANT * (273.15 + &self.temperature))
    }

    pub fn get_hour(&self) -> &f32 {
        &self.time
    }

    pub fn second_passed(&self) -> bool {
        self.second_passed
    }
}

fn step_time(mut world_state: ResMut<WorldState>, time: Res<Time>) {
    world_state.time += time.delta_secs();
    world_state.second_passed = (world_state.time % 1.0).ceil() as u8 == 0;
    if world_state.time > 24.0 {
        world_state.time = 0.0;
    }
}

fn step_projectiles(
    time: Res<Time>,
    mut projectiles: Query<(&mut Bullet, &mut Transform)>
) {
    let delta_time = time.delta_secs();

    for (mut ballistics, mut transform) in projectiles.iter_mut() {
        ballistics.step(delta_time); 
        transform.translation = ballistics.position;
    }
}
