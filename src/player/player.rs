use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::{camera::{update_camera_controller, CameraController}, player_input::{apply_movement, handle_player_input}, state::player_state::PlayerState};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, apply_movement.in_set(PhysicsSet::Writeback))
            .add_systems(Update, (update_camera_controller, handle_player_input));
    }
}


#[derive(Component)]
pub struct Player {
    pub health: u8,

    pub gravity: f32,
    pub speed: f32,
    pub speed_multiplier: f32,
    pub momentum: Vec3,
    pub direction: Vec3,

    pub state: PlayerState
}

impl Default for Player {
    fn default() -> Self {
        Player { 
            health: 100, 

            //Physics/motion
            gravity: 9.8, // m/s
            speed: 50_f32, // Km/h, douvle for running
            speed_multiplier: 2_f32,
            direction: Vec3::ZERO,
            momentum: Vec3::ZERO, 

            state: PlayerState::default()
        }
        
    }
}

fn spawn_player(mut commands: Commands) {
    let camera_entity = commands.spawn((
        Camera3d::default(), 
        Transform::from_xyz(0., 0., 0.),
        CameraController {
            sensitivity: 0.030,
            rotation: Vec2::ZERO,
            rotation_lock: 90_f32,
        })
    ).id();

    let player_entity = commands.spawn((
        Player::default(),
        Transform::from_xyz(128., 32., 128.),
        Collider::cuboid(1., 10., 1.),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            ..Default::default()
        }
    )).id();

    commands.entity(player_entity).add_child(camera_entity);
}
