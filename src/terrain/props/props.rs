use bevy::prelude::*;
use bevy_rapier3d::{prelude::{Collider, RigidBody}};

use crate::noise::{perlin_cpu::PerlinCPU, poisson_disc::PoissonDisc};

pub struct PropPlugin;

impl Plugin for PropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_props);
    }
}

fn load_props(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>,  mut meshes: ResMut<Assets<Mesh>>) {
    let perlin = PerlinCPU::new(1, 0.001, 4, 2.0, 0.5);
    let mut poisson = PoissonDisc::new(10., Vec2::new(128., 128.), 10);
    let points = poisson.generate_points();
    let brown = materials.add(StandardMaterial { base_color: Color::srgb_u8(165, 42, 42), perceptual_roughness: 0.5, ..default() });

    for point in points {
        let x = point.x + 2048.0;
        let z = point.y + 2048.0;
        let y = perlin.from_fractal(x, z) + 40.0;
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cylinder::new(0.25, 20.0)))),
            MeshMaterial3d(brown.clone()),
            Collider::cylinder(10.0, 0.25),
            RigidBody::Fixed,
            Transform::from_xyz(x, y, z),
        ));
    }
}
