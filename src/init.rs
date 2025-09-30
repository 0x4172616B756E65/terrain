use std::collections::HashSet;

use bevy::{app::Plugin, color::palettes::tailwind::GREEN_200, pbr::light_consts::lux::OVERCAST_DAY, prelude::*};
use bevy_rapier3d::prelude::Collider;

use crate::{noise::perlin::Perlin, player::player::Player, simulation::physics::WorldState, terrain::{chunks::{Chunkbase, RenderDistance, RenderedChunks}, grid::{ChunkRadius, CurrentChunk}}};

pub struct Init;

impl Plugin for Init {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .insert_resource(ChunkRadius::default())
            .insert_resource(RenderDistance(16))
            .insert_resource(RenderedChunks::default())
            .insert_resource(PreviousRadius::default())
            .insert_resource(WorldState::default())
            .insert_resource(Perlin::new(1, 0.01, 4, 2., 0.5))
            .add_systems(Startup, (init_resources, setup_scene))
            .add_systems(Update, load_map);
    }
}

#[derive(Resource, Default)]
struct PreviousRadius(pub HashSet<(i32, i32)>);

#[derive(Component)]
struct CustomUV;

fn init_resources(mut commands: Commands, perlin: Res<Perlin>) {
    let chunkbase: Chunkbase = Chunkbase::new_with_mesh(256, 256, &perlin, true);

    commands.spawn(Collider::cuboid(8192., 0., 8192.));
    commands.insert_resource(chunkbase);
}

fn setup_scene(mut commands: Commands) {
    let light_transform = Transform::from_xyz(4096., 1024., 4096.).looking_at(Vec3::new(4096., 0., 4096.), Vec3::Y);

    commands.spawn((DirectionalLight {
        illuminance: OVERCAST_DAY, 
        ..Default::default()
    }, light_transform));

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn load_map(
    chunkbase: Res<Chunkbase>,
    render_distance: Res<RenderDistance>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    mut previous_radius: ResMut<PreviousRadius>,
    mut rendered_chunks: ResMut<RenderedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<CurrentChunk>,
) {
    let mut player = player_query.single_mut().unwrap();
    let stone = materials.add(StandardMaterial { base_color: GREEN_200.into(), perceptual_roughness: 0.5, ..default() });
    
    for CurrentChunk((cx, cy)) in events.read() {
        let load_raw: HashSet<(i32, i32)> =
            get_render_radius(*cx, *cy, render_distance.0).iter().cloned().collect();

        for coord in previous_radius.0.difference(&load_raw) {
            if let Some(entity) = rendered_chunks.0.remove(coord) {
                commands.entity(entity).despawn();
            }
        }

        for coord in load_raw.difference(&previous_radius.0) {
            if let Some(chunk) = chunkbase.load_chunk(coord) {
                let handle = meshes.add(chunk.get_mesh().as_ref().unwrap().clone());

                let entity = commands.spawn((
                    Mesh3d(handle),
                    MeshMaterial3d(stone.clone()),
                    CustomUV,
                )).id();

                rendered_chunks.0.insert(*coord, entity);
            }
        }

        previous_radius.0 = load_raw;
        player.current_chunk = CurrentChunk((*cx, *cy));
    }

    if render_distance.is_changed() {
        let (cx, cy) = player.current_chunk.0;
        let load_raw: HashSet<(i32, i32)> = 
            get_render_radius(cx, cy, render_distance.0).iter().cloned().collect();

        for coord in previous_radius.0.difference(&load_raw) {
            if let Some(entity) = rendered_chunks.0.remove(coord) {
                commands.entity(entity).despawn();
            }
        }

        for coord in load_raw.difference(&previous_radius.0) {
            if let Some(chunk) = chunkbase.load_chunk(coord) {
                let handle = meshes.add(chunk.get_mesh().as_ref().unwrap().clone());

                let entity = commands.spawn((
                    Mesh3d(handle),
                    MeshMaterial3d(stone.clone()),
                    CustomUV,
                )).id();

                rendered_chunks.0.insert(*coord, entity);
            }
        }

        previous_radius.0 = load_raw;
    }
}

pub fn get_render_radius(cx: i32, cy: i32, radius: i32) -> Vec<(i32, i32)> {
        let mut chunks = Vec::with_capacity((radius * 2 + 1).pow(2) as usize);
        let radius_sq = radius * radius;

        for y in -radius..=radius {
            let y_sq = y * y;
            for x in -radius..=radius {
                if x * x + y_sq <= radius_sq {
                    let chunk_coords = (cx.wrapping_add(x), cy.wrapping_add(y));
                        chunks.push(chunk_coords);
                }
            }
        }
    chunks
}
