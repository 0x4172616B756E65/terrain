use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{player::{camera_controller::CameraController, config::player_config::{InputBinding, PlayerAction::{self, *}}, player::Player, player_attack::DebugShootEvent}, simulation::world::{WorldState, GRAVITY}, terrain::chunks::RenderDistance};

pub fn handle_player_input(
    mut player_query: Query<(&mut Player, &Transform)>, 
    camera_query: Query<&CameraController>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut render_distance: ResMut<RenderDistance>,
    mut debug_shoot: EventWriter<DebugShootEvent>
) {
    let (mut player, transform) = player_query.single_mut().unwrap();
    let camera = camera_query.single().unwrap();

    let yaw = -camera.rotation.y.to_radians() - 90.0_f32.to_radians();
    let pitch = camera.rotation.x.to_radians();

    let forwards = Vec3::new(
        pitch.cos() * yaw.cos(),
        pitch.sin(),
        pitch.cos() * yaw.sin(),
    );

    let keymap = player.config.keymap.clone();
    player.direction = Vec3::ZERO;
    player.speed_multiplier = 1.0;

    let mut apply_action = |action: PlayerAction| {
        match action {
            MoveForwards => player.direction.x += 1.,
            MoveBackwards => player.direction.x -= 1.,
            MoveRightwards => player.direction.z += 1.,
            MoveLeftwards => player.direction.z -= 1.,

            FlyUpwards => player.direction.y += 1.,
            FlyDownwards => player.direction.y -= 1.,

            MoveSprinting => player.speed_multiplier = 2.0,

            DebugShootBullet => { let _ = debug_shoot.write(DebugShootEvent((*transform, forwards))); }, 

            DebugIncreaseRenderDistance => render_distance.0 += 1,
            DebugDecreaseRenderDistance => { render_distance.0 = render_distance.0.saturating_sub(1) },

            DebugToggleFlight => player.state.debug_flying ^= true,
            _ => {}
        }
    };

    for pressed_key in keys.get_pressed() {
        if let Some(action) = keymap.get(&InputBinding::Key((*pressed_key).into())) {
            apply_action(*action);
        }
    }

    for pressed_button in [MouseButton::Left, MouseButton::Right] {
        if mouse_buttons.just_pressed(pressed_button) {
            if let Some(action) = keymap.get(&InputBinding::MouseButton(pressed_button.into())) {
                apply_action(*action);
            }
        }
    }

    for ev in scroll_events.read() {
        if ev.y > 0.0 {
            if let Some(action) = keymap.get(&InputBinding::MouseWheelUp) {
                apply_action(*action);
            }
        } else if ev.y < 0.0 {
            if let Some(action) = keymap.get(&InputBinding::MouseWheelDown) {
                apply_action(*action);
            }
        }
    }
}

pub fn apply_player_movement(
    time: Res<Time<Fixed>>,
    camera_query: Query<&CameraController>,
    player_query: Query<(
        &mut Player,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>
    )>,
    world_state: Res<WorldState>,
) {
    let camera = camera_query.single().unwrap();

    let yaw_radians = -camera.rotation.y.to_radians() - 90_f32.to_radians();
    let camera_forward = Vec3::new(f32::cos(yaw_radians), 0.0, f32::sin(yaw_radians));
    let camera_right = Vec3::new(-camera_forward.z, 0.0, camera_forward.x);

    for (mut player, mut controller, output) in player_query {
        let mut movement = Vec3::ZERO;
        let delta = time.timestep().as_secs_f32();
        movement += camera_forward * player.direction.x;
        movement += camera_right * player.direction.z;

        movement.y += player.direction.y;

        if movement.length_squared() > 0.0 {
            movement = movement.normalize() * player.speed * player.speed_multiplier;
            player.momentum = movement;
        }


        if player.state.debug_flying {
            if player.direction.y != 0.0 {
                player.momentum.y += player.direction.y * delta;
            }
        } else if let Some(output) = output {
            if !output.grounded { 
                info!("Not grounded");
                player.momentum.y -= GRAVITY * delta; 
            } else if player.direction == Vec3::ZERO {
                player.momentum = Vec3::ZERO;
            }
        }

        controller.translation = Some(player.momentum * delta);
    }
}
