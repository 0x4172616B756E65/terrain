use bevy::math::{primitives::Triangle3d, Vec2, Vec3};

use crate::noise::perlin::Vector;

struct Vertex {
    position: Vec3,
    normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3) -> Self { Vertex { position, normal } }
}

fn populate_grid(x: f32, y: f32, z: f32) {
    let vertex = Vertex {
        position: Vec3::new(x as f32, z, y as f32),
        normal: Vec3::ZERO, // calculate later
        //uv: Vec2::new(x / (width-1), y / (height-1))
    };
    /*
    let vertex = (x, y, z);
    let v00 = Vec3::new(x, y, z);
    let v01 = Vec3::new(x + 1., y, z);
    let v10 = Vec3::new(x, y + 1., z);
    let v11 = Vec3::new(x + 1., y + 1., z);
    let triangle_1: Triangle3d = Triangle3d { vertices: [v00, v01, v10] };
    let triangle_2: Triangle3d = Triangle3d { vertices: [v11, v01, v10] };

    let edge_1: Vec2 = Vec2::new(v01.x - v00.x, v01.y - v00.y);
    let edge_2: Vec2 = Vec2::new(v11.x - v10.x, v11.y - v10.y);
    let normal = edge_1.perp_dot(edge_2);
    */
}
