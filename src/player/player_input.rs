use bevy::{input::mouse::MouseWheel, prelude::*, reflect::Enum};
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{init::Physics, player::{camera_controller::CameraController, config::player_config::{InputBinding, PlayerAction::{self, *}, PressKind}, player::Player, player_attack::DebugShootEvent, player_state::ToggleInventory}, simulation::world::{WorldState, GRAVITY}, terrain::chunks::RenderDistance};

pub fn handle_player_input(
    mut player_query: Query<(&mut Player, &Transform)>, 
    camera_query: Query<&CameraController>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut render_distance: ResMut<RenderDistance>,
    mut debug_shoot: EventWriter<DebugShootEvent>,
    mut toggle_inventory: EventWriter<ToggleInventory>,
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

            OpenInventory => { let _ = toggle_inventory.write(ToggleInventory); player.state.inventory_open ^= true; },

            DebugShootBullet => { let _ = debug_shoot.write(DebugShootEvent((*transform, forwards))); }, 

            DebugIncreaseRenderDistance => render_distance.0 += 1,
            DebugDecreaseRenderDistance => { render_distance.0 = render_distance.0.saturating_sub(1) },

            DebugToggleFlight => player.state.debug_flying ^= true,
            _ => {}
        }
    };

    for pressed_key in keys.get_pressed() {
        if let Some((action, PressKind::Momentary)) = keymap.get(&InputBinding::Key((*pressed_key).into())) {
            apply_action(*action);
        }
    }

    for pressed_key in keys.get_just_pressed() {
        if let Some((action, PressKind::MonoStable)) = keymap.get(&InputBinding::Key((*pressed_key).into())) {
            apply_action(*action);
        }
    }

    for pressed_button in mouse_buttons.get_just_pressed() {
        if let Some((action, PressKind::MonoStable)) = keymap.get(&InputBinding::MouseButton((*pressed_button).into())) {
            apply_action(*action);
        }
    }

    for ev in scroll_events.read() {
        if ev.y > 0.0 {
            if let Some((action, PressKind::MonoStable)) = keymap.get(&InputBinding::MouseWheelUp) {
                apply_action(*action);
            }
        } else if ev.y < 0.0 {
            if let Some((action, PressKind::MonoStable)) = keymap.get(&InputBinding::MouseWheelDown) {
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
    mut text_query: Query<&mut Text, With<Physics>>
) {
    let camera = camera_query.single().unwrap();
    let mut text = text_query.single_mut().unwrap();

    let yaw_radians = -camera.rotation.y.to_radians() - 90_f32.to_radians();
    let camera_forward = Vec3::new(f32::cos(yaw_radians), 0.0, f32::sin(yaw_radians));
    let camera_right = Vec3::new(-camera_forward.z, 0.0, camera_forward.x);

    for (mut player, mut controller, output) in player_query {
        let mut movement = Vec3::ZERO;
        let delta = time.timestep().as_secs_f32();
        movement += camera_forward * player.direction.x;
        movement += camera_right * player.direction.z;

        //movement.y += player.direction.y;

        if movement.length_squared() > 0.0 {
            movement = movement.normalize() * player.speed * player.speed_multiplier;
            player.momentum.x = movement.x;
            player.momentum.z = movement.z;
        }

        let mut grounded = "";
        if player.state.debug_flying {
            if player.direction.y != 0.0 {
                player.momentum.y += player.direction.y * delta;
            }
        } else if let Some(output) = output {
            grounded = match output.grounded {
                true => "Grounded",
                false => "Not grounded"
            };
            if !output.grounded { 
                player.momentum.y -= GRAVITY * delta; 
            } else {
                player.momentum.y = 0.0;
            } 

            if player.direction == Vec3::ZERO {
                player.momentum.x = 0.0;
                player.momentum.z = 0.0

            }
        }


        text.clear();
        text.push_str(format!("{:?} | {grounded}", player.momentum).as_str());

        controller.translation = Some(player.momentum * delta);
    }
}
