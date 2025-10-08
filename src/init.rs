use std::collections::HashSet;
#[cfg(feature = "debug")]
use std::time::Instant;

use bevy::{app::Plugin, color::palettes::tailwind::GREEN_200, pbr::light_consts::lux::OVERCAST_DAY, prelude::*, tasks::{block_on, AsyncComputeTaskPool}};
use bevy_rapier3d::prelude::Collider;

use crate::{noise::perlin::Perlin, player::{camera_controller::CameraController, player::Player}, simulation::physics::WorldState, terrain::{chunks::{Chunkbase, RenderDistance, RenderedChunks}, grid::{ChunkRadius, CurrentChunk}}};

pub struct Init;

impl Plugin for Init {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .insert_resource(ChunkRadius::default())
            .insert_resource(RenderDistance(16))
            .insert_resource(RenderedChunks::default())
            .insert_resource(block_on(Perlin::new(1, 0.1)).unwrap())
            .insert_resource(WorldState::default())
            .add_systems(Startup, (init_resources, setup_scene))
            .add_systems(Update, update_compass);
    }
}



#[derive(Component)]
pub struct DebugText;

#[derive(Component)]
struct Compass;

fn init_resources(mut commands: Commands, perlin: Res<Perlin>) {
    let chunkbase: Chunkbase = Chunkbase::new(&perlin);

    #[cfg(feature = "debug")]
    info!("Time to load chunkbase: {:?}\n Current chunks: {}", start.elapsed(), 16_000);

    commands.spawn(Collider::cuboid(8192., 0., 8192.));
    commands.insert_resource(chunkbase);
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center, 
            width: Val::Percent(100.0),
            top: Val::Px(12.0),
            ..Default::default()
        },
    ).with_child((Text::new(""), Compass));

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    )).insert(DebugText);
}

fn update_compass(mut compass_query: Query<&mut Text, With<Compass>>, camera_query: Query<&mut CameraController>) {
    let mut compass = compass_query.single_mut().unwrap();
    let camera_controller = camera_query.single().unwrap();
    let rotation_radians = -camera_controller.rotation.y.to_radians() - 90.0_f32.to_radians();
    let rotation_degrees = (((rotation_radians.to_degrees() % 360.0) + 360.0) % 360.0) as usize;

    let yaw = match rotation_degrees {
        0 => String::from("N"),
        90 => String::from("W"),
        180 => String::from("S"),
        270 => String::from("E"),
        _ => format!("{rotation_degrees}")
    };

    compass.clear();
    compass.push_str(format!("{yaw}").as_str()); 
}



