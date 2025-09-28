
use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{Extent3d, TextureDimension, TextureFormat}}};

use crate::noise::perlin::Perlin;

struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl RGBA {
    const RED: RGBA = RGBA { red: 255, green: 0, blue: 0, alpha: 255 };
    const GREEN: RGBA = RGBA { red: 0, green: 255, blue: 0, alpha: 255 };
    const BLUE: RGBA = RGBA { red: 0, green: 0, blue: 255, alpha: 255 };

    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self { RGBA { red, green, blue, alpha }}
}

#[rustfmt::skip]
pub fn create_cube_mesh() -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,

        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )

    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )

    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )

    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}

/*
fn setup_2d(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let perlin = Perlin::new();
    let width: u32 = 1024;
    let height: u32 = 1024;
    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    let scale = 0.01;

    for y in 0..height {
        for x in 0..width {
        let fx = x as f32 * scale;
        let fy = y as f32 * scale;

        let sample = perlin.from_fractal(seed, fx, fy, 3, 2.0, 0.5);

        let value = ((sample + 1.0) * 0.5 * 255.0) as u8;
        //let pixel: RGBA = RGBA::new(value, value, value, 255);

        let pixel = match value {
            0..128 => RGBA::BLUE,
            _ => RGBA::GREEN
        };

        pixels.push(pixel.red);
        pixels.push(pixel.green);
        pixels.push(pixel.blue);
        pixels.push(pixel.alpha);
        }
    } 

    info!("{:?}", pixels);

    let extent = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let image = Image::new_fill(extent, TextureDimension::D2, &pixels, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD);
    let image_handle = images.add(image);
    commands.spawn(Camera2d);

    commands.spawn(Sprite {
        image: image_handle,
        ..default()
    });
}*/

