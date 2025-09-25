use bevy::{asset::RenderAssetUsages, image::Image, log::tracing_subscriber, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use bevy::prelude::*;
use terrain::noise::perlin::{Perlin, Seed};
use tracing::info;

fn main() {
    tracing_subscriber::fmt().init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let perlin = Perlin::new();
    let seed = Seed::new(1); 
    let width: u32 = 1024;
    let height: u32 = 1024;
    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    let scale = 0.10;

    for y in 0..height {
        for x in 0..width {
        let fx = x as f32 * scale;
        let fy = y as f32 * scale;

        let sample = perlin.from_fractal(seed, fx, fy, 3, 2.0, 0.5);

        let value = ((sample + 1.0) * 0.5 * 255.0) as u8;
        pixels.push(value);
        pixels.push(value);
        pixels.push(value);
        pixels.push(255);
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
