use std::collections::HashSet;

use bevy::prelude::*;

use crate::{player::player::Player, terrain::chunks::{Chunkbase, RenderDistance, RenderedChunks}};

pub struct GridPlugin;

#[derive(Resource, Default)]
struct LastChunk((i32, i32));

#[derive(Event)]
pub struct CurrentChunk(pub (i32, i32));

#[derive(Resource, Default)]
struct PreviousRadius(pub HashSet<(i32, i32)>);

#[derive(Component)]
struct CustomUV;

#[derive(Resource, Default)]
pub struct ChunkRadius(pub Vec<Entity>);

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LastChunk>()
            .insert_resource(PreviousRadius::default())
            .add_event::<CurrentChunk>()
            .add_systems(Update, (enter_chunk_event, load_map))
        ;
    }
}

fn enter_chunk_event(
    player_query: Query<(&Player, &Transform)>,
    mut last_chunk: ResMut<LastChunk>,
    mut events: EventWriter<CurrentChunk>
) {
    let (_, transform) = player_query.single().unwrap();
    let current_chunk = (
        (transform.translation.x / 32.0).floor() as i32,
        (transform.translation.z / 32.0).floor() as i32,
    );

    if current_chunk != last_chunk.0 {
        events.write(CurrentChunk(current_chunk));
        last_chunk.0 = current_chunk;
    }
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
    let stone = materials.add(StandardMaterial { base_color: Color::srgb_u8(0, 180, 20), perceptual_roughness: 0.5, ..default() });
    
    let mut update_chunks = |load_raw: HashSet<(i32, i32)>| {
        for coord in previous_radius.0.difference(&load_raw) {
            if let Some(entity) = rendered_chunks.0.remove(coord) {
                commands.entity(entity).despawn();
            }
        }

        for coord in load_raw.difference(&previous_radius.0) {
            if let Some(chunk) = chunkbase.get_chunk(coord) {
                let chunk_entity = commands.spawn((
                    Mesh3d(meshes.add(chunk.mesh.clone())),
                    chunk.transform,
                    MeshMaterial3d(stone.clone()),
                    CustomUV,
                )).id();

                rendered_chunks.0.insert(*coord, chunk_entity);
            }
        }

        previous_radius.0 = load_raw;
    };

    for CurrentChunk((cx, cy)) in events.read() {
        let load_raw: HashSet<(i32, i32)> = get_circle_area(*cx, *cy, render_distance.0).iter().cloned().collect();

        update_chunks(load_raw);

        player.current_chunk = CurrentChunk((*cx, *cy));
    }

    if render_distance.is_changed() {
        let (cx, cy) = player.current_chunk.0;
        let load_raw: HashSet<(i32, i32)> = get_circle_area(cx, cy, render_distance.0).iter().cloned().collect();

        update_chunks(load_raw);
    }
}

pub fn get_circle_area(cx: i32, cy: i32, radius: i32) -> Vec<(i32, i32)> {
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
