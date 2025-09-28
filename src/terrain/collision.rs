/*
use bevy::prelude::*;
use bevy_rapier3d::{na::{DMatrix, Vector3}, parry::shape::HeightField, prelude::*};

fn load_collision() {
    let hf = HeightField::new(heights, scale);
 
}

fn parse_heightmap(heightmap: Vec<u32>) {
    let width = 128;
    let height = 128;
    let heightmap: Vec<f32> = (0..width*height)
        .map(|i| (i % width) as f32 * 0.1) // example: just a slope
        .collect();

    // Convert to DMatrix
    // DMatrix::from_row_slice(rows, cols, &[...])
    let heights_matrix = DMatrix::from_row_slice(height, width, &heightmap);

    // Scale between points (distance between vertices in X, Y, Z)
    let scale = Vector3::new(1.0, 1.0, 1.0); // 1 unit per grid in X/Z, 1 unit height in Y

    let collider = Collider::heightfield(heights_matrix, scale);

    // Spawn terrain
    commands.spawn((
        collider,
        RigidBody::Fixed,
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}*/
