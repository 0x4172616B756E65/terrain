use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{player::{camera_controller::CameraController, player::Player, player_attack::DebugShootEvent, config::player_config::{InputBinding, PlayerAction::{self, *}}}, terrain::chunks::RenderDistance};

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
            DebugDecreaseRenderDistance => render_distance.0 -= 1,
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
