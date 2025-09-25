use bevy::{asset::RenderAssetUsages, image::Image, log::tracing_subscriber, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{Extent3d, TextureDimension, TextureFormat}}};
use bevy::prelude::*;
use terrain::noise::perlin::{Perlin, Seed};
use tracing::info;

#[derive(Component)]
struct CustomUV;

struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2, // optional
}

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

fn main() {
    tracing_subscriber::fmt().init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_3d)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let perlin = Perlin::new();
    let seed = Seed::new(1); 
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
}

fn setup_3d(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let perlin = Perlin::new();
    let seed = Seed::new(1);
    let width = 256u32;
    let height = 256u32;
    let scale = 0.1;


    let mut vertices = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let fx = x as f32 * scale;
            let fy = y as f32 * scale;
            let z = (perlin.from_fractal(seed, fx, fy, 4, 2.0, 0.5) + 0.5).powi(4) * 10.;
            info!("{z}");

            vertices.push([x as f32, z, y as f32]); // [x, height, y]
        }
    }

    let mut indices = Vec::with_capacity(((width - 1) * (height - 1) * 6) as usize);
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let i0 = x + y * width;
            let i1 = i0 + 1;
            let i2 = i0 + width;
            let i3 = i2 + 1;

            // Two triangles per quad
            indices.push(i0);
            indices.push(i3);
            indices.push(i1);

            indices.push(i0);
            indices.push(i2);
            indices.push(i3);
        }
    }

    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone())
        .with_inserted_indices(Indices::U32(indices));

    let mesh_handle = meshes.add(mesh);

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("28221B").unwrap().into(),
            perceptual_roughness: 1.0,
            ..default()
        })),
        CustomUV
    ));
}

fn setup_camera(mut commands: Commands) {
    let camera_and_light_transform =
        Transform::from_xyz(-100., 100., -100.).looking_at(Vec3::new(128., -2., 128.), Vec3::Y);

    commands.spawn((Camera3d::default(), camera_and_light_transform));
    commands.spawn((PointLight::default(), camera_and_light_transform));

}

fn create_cube_mesh() -> Mesh {
     Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![

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
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}
