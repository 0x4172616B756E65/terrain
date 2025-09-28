use bevy::{prelude::*, render::camera::CameraProjection};
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::player::{camera::CameraController, player::Player, player_state::{PlayerAction::*, PlayerState}};

pub fn handle_player_input(mut player_query: Query<&mut Player>, keys: Res<ButtonInput<KeyCode>>) {
    let mut player = player_query.single_mut().unwrap();
    player.direction = Vec3::ZERO;
    player.speed_multiplier = 1.0;

    for pressed_key in keys.get_pressed() {
        if let Some(input) = player.state.keymap.get(pressed_key) {
            match input {
                MOVE_FORWARDS => player.direction.x += 1.,
                MOVE_BACKWARDS => player.direction.x -= 1.,
                MOVE_RIGHTWARDS => player.direction.z += 1.,
                MOVE_LEFTWARDS => player.direction.z -= 1.,

                FLY_UPWARDS => player.direction.y += 1.,
                FLY_DOWNWARDS => player.direction.y -= 1.,

                MOVE_SPRINTING => player.speed_multiplier = 2_f32,
                _ => {}
            }
        }
    }
}

pub fn apply_movement(
    time: Res<Time<Fixed>>, 
    camera_query: Query<&CameraController>,
    player_query: Query<(
        &mut Player, 
        &mut KinematicCharacterController, 
        Option<&KinematicCharacterControllerOutput>
    )> ) {
    let camera = camera_query.single().unwrap();

    for (mut player, mut controller, _controller_output) in player_query {
        let camera_rotation_converted = -camera.rotation.y.to_radians() - 90_f32.to_radians();
        let forwards = Vec3::new(f32::cos(camera_rotation_converted), 0.0, f32::sin(camera_rotation_converted));
        let rightwards = Vec3::new(-forwards.z, 0.0, forwards.x);

        let mut movement = Vec3::ZERO;
        movement += forwards * player.direction.x;
        movement += rightwards * player.direction.z;
        movement.y += player.direction.y;

        if movement.length_squared() > 0.0 {
            movement = movement.normalize() * player.speed * player.speed_multiplier;
            player.momentum = movement;
        } else {
            player.momentum = Vec3::ZERO;
        }

        controller.translation = Some(player.momentum * time.timestep().as_secs_f32());
    }
}
