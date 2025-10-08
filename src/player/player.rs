use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{player::{camera_controller::{update_camera_controller, CameraController}, config::player_config::PlayerConfig, player_attack::debug_shoot_bullet, player_input::{apply_player_movement, handle_player_input}, player_state::PlayerState}, terrain::grid::CurrentChunk};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, apply_player_movement.in_set(PhysicsSet::Writeback))
            .add_systems(Update, (update_camera_controller, handle_player_input, debug_shoot_bullet));
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

    pub current_chunk: CurrentChunk,
    pub config: PlayerConfig,
    pub state: PlayerState,
}

impl Default for Player {
    fn default() -> Self {
        Player { 
            health: 100, 

            //Physics/motion
            gravity: 9.8, // m/s
            speed: 5_f32, // Km/h, douvle for running
            speed_multiplier: 2_f32,
            direction: Vec3::ZERO,
            momentum: Vec3::ZERO, 

            current_chunk: CurrentChunk((0, 0)),
            config: PlayerConfig::default(),
            state: PlayerState::default(),
        }
        
    }
}

fn spawn_player(mut commands: Commands) {
    let camera_entity = commands.spawn((
        Camera3d::default(), 
        Transform::from_xyz(0., 0.9, 0.),
        Projection::from(PerspectiveProjection {
            fov: 120_f32,
            ..default()
        }),
        CameraController {
            sensitivity: 0.030,
            rotation: Vec2::ZERO,
            rotation_lock: 90_f32,
        })
    ).id();

    let player_entity = commands.spawn((
        Player::default(),
        Transform::from_xyz(2048., 32., 2048.),
        Collider::cuboid(0.5, 1.8, 0.25),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            ..Default::default()
        }
    )).id();

    commands.entity(player_entity).add_child(camera_entity);
}
