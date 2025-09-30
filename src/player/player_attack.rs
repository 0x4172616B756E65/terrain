use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::ecs::event::{Event, EventReader};

use crate::simulation::ballistics::ammunition::{Ballistics, Bullet};
use crate::simulation::physics::WorldState;

#[derive(Event)]
pub struct DebugShootEvent(pub (Transform, Vec3));

#[derive(Resource)]
pub struct TimedAction {
    timer: Timer,
}

pub fn debug_shoot_bullet(
    time: Res<Time>, 
    world_state: Res<WorldState>,
    mut events: EventReader<DebugShootEvent>, 
    mut commands: Commands,  
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for DebugShootEvent((transform, direction)) in events.read() {
        let bullet = Bullet::new_nine_mm(*direction, 360., 8_900., &world_state, Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z));
        info!("Bullet shot with velocity: {:?}, magnitude: {}", bullet.instant_velocity(), bullet.instant_velocity().length());
        let mesh = meshes.add(Sphere::new(0.4).mesh().ico(5).unwrap());

        commands.spawn((
            bullet, 
            Transform::from_xyz(transform.translation.x, transform.translation.y, transform.translation.z),
            MeshMaterial3d(materials.add(StandardMaterial { base_color: RED.into(), perceptual_roughness: 0.5, ..default() })),
            Mesh3d(mesh.clone())
        ));
    } 
}
fn _debug_shoot_bullet_with_inventory() {}
