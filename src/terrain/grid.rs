use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{player::player::Player, terrain::chunks::{Chunkbase, RenderDistance, RenderedChunks, CHUNK_HEIGHT, CHUNK_WIDTH}};

pub struct GridPlugin;

#[derive(Resource, Default)]
struct LastChunk((i32, i32));

#[derive(Event)]
pub struct CurrentChunk(pub (i32, i32));

#[derive(Resource, Default)]
struct RenderRadius(pub HashSet<((i32, i32), u32)>);

#[derive(Resource, Default)]
struct LODRadiusOld(pub HashSet<(i32, i32)>);

#[derive(Resource, Default)]
pub struct ChunkRadius(pub Vec<Entity>);

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LastChunk>()
            .insert_resource(RenderRadius::default())
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
        (transform.translation.x / CHUNK_WIDTH as f32).floor() as i32,
        (transform.translation.z / CHUNK_HEIGHT as f32).floor() as i32,
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
    mut render_radius: ResMut<RenderRadius>,
    mut rendered_chunks: ResMut<RenderedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<CurrentChunk>,
) {
    let mut player = player_query.single_mut().unwrap();
    let green = materials.add(StandardMaterial { base_color: Color::srgb_u8(20, 180, 20), perceptual_roughness: 0.5, ..default() });
    let yellow = materials.add(StandardMaterial { base_color: Color::srgb_u8(180, 180, 20), perceptual_roughness: 0.5, ..default() });
    let red = materials.add(StandardMaterial { base_color: Color::srgb_u8(180, 20, 20), perceptual_roughness: 0.5, ..default() });
    
    let mut update_chunks = |load_raw: HashSet<((i32, i32), u32)>| {
        for chunk_info in render_radius.0.difference(&load_raw) {
            if let Some(entity) = rendered_chunks.0.remove(&chunk_info.0) {
                commands.entity(entity).despawn();
            }
        }

        for chunk_info in load_raw.difference(&render_radius.0) {
            if let Some(chunk) = chunkbase.get_chunk(&chunk_info.0) {
                let chunk_entity;
                if chunk_info.1 == 0 {
                    chunk_entity = commands.spawn((
                        Mesh3d(meshes.add(chunk.mesh.clone())), 
                        MeshMaterial3d(green.clone()), 
                        RigidBody::Fixed,
                        chunk.transform,
                    )).with_child((
                        RigidBody::Fixed,
                        chunk.collider.clone(),
                        Transform::from_xyz(64.0, 0.0, 64.0)
                                .with_rotation(Quat::from_rotation_y(90_f32.to_radians()))
                                //FIXME.with_scale(Vec3::new(1.0, 1.0, -1.0))
                    )).id();
                } else if chunk_info.1 == 2 {
                    chunk_entity = commands.spawn((
                        Mesh3d(meshes.add(chunk.mesh_2.clone())),
                        MeshMaterial3d(yellow.clone()),
                        chunk.transform, 
                    )).id();
                }
                else { 
                    chunk_entity = commands.spawn((
                        Mesh3d(meshes.add(chunk.mesh_4.clone())),
                        MeshMaterial3d(red.clone()),
                        chunk.transform,
                    )).id();
                }

                rendered_chunks.0.insert(chunk_info.0, chunk_entity);
            }
        }

        render_radius.0 = load_raw;
    };

    for CurrentChunk((cx, cy)) in events.read() {
        let load_raw: HashSet<((i32, i32), u32)> = get_circle_area(*cx, *cy, render_distance.0 as i32, player.config.lod_radius).iter().cloned().collect();

        update_chunks(load_raw);

        player.current_chunk = CurrentChunk((*cx, *cy));
    }

    if render_distance.is_changed() {
        let (cx, cy) = player.current_chunk.0;
        let load_raw: HashSet<((i32, i32), u32)> = get_circle_area(cx, cy, render_distance.0 as i32, player.config.lod_radius).iter().cloned().collect();

        update_chunks(load_raw);
    }

}

pub fn get_circle_area(cx: i32, cy: i32, radius: i32, lod_radius: u32) -> Vec<((i32, i32), u32)> {
    let mut chunks = Vec::with_capacity((radius * 2 + 1).pow(2) as usize);
    let radius_sq = radius * radius;

    for y in -radius..=radius {
        let y_sq = y * y;
        for x in -radius..=radius {
            if x * x + y_sq <= radius_sq {
                let chunk_coords = (cx.wrapping_add(x), cy.wrapping_add(y));
                    let dx = chunk_coords.0 - cx;
                    let dy = chunk_coords.1 - cy;
                    let distance = ((dx*dx + dy*dy) as f32).sqrt() as u32;

                    let mut lod = 4;
                    if distance < lod_radius { lod = 0; } 
                    else if distance < lod_radius * 2 { lod = 2; } 

                    chunks.push((chunk_coords, lod));
            }
        }
    }
    chunks
}
