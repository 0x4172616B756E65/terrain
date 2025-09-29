use bevy::{color::palettes::{css::WHITE, tailwind::GREEN_500}, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, log::tracing_subscriber, pbr::light_consts::lux::{FULL_DAYLIGHT}};
use bevy::prelude::*;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, KinematicCharacterController}, render::RapierDebugRenderPlugin};
use terrain::{noise::perlin::Perlin, player::{cursor::CursorPlugin, player::{Player, PlayerPlugin}}, simulation::physics::BallisticsPlugin, terrain::{chunks::{Chunkbase, RenderDistance, RenderedChunks}, grid::{ChunkRadius, CurrentChunk, GridPlugin}}};
use std::collections::HashSet;

#[derive(Component)]
struct CustomUV;

fn main() {
    tracing_subscriber::fmt().init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(BallisticsPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default()) 
        .insert_resource(Perlin::new(1, 0.01, 4, 2., 0.5))
        .insert_resource(ChunkRadius::default())
        .insert_resource(RenderDistance::default())
        .insert_resource(RenderedChunks::default())
        .insert_resource(PreviousRadius::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CursorPlugin)
        //.add_plugins(PropPlugin)
        .add_systems(Startup, (setup_scene, init_resources))
        .add_systems(Update, (load_chunks, debug))
        .run();
}

#[derive(Resource, Default)]
struct PreviousRadius(pub HashSet<(i32, i32)>);

fn init_resources(mut commands: Commands, perlin: Res<Perlin>) {
    let chunkbase: Chunkbase = Chunkbase::new_with_mesh(64, 64, &perlin, true);

    commands.spawn(Collider::cuboid(1000., 0., 1000.));
    commands.insert_resource(chunkbase);
}

fn load_chunks(
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
    let stone = materials.add(StandardMaterial { base_color: GREEN_500.into(), perceptual_roughness: 0.5, ..default() });
    
    for CurrentChunk((cx, cy)) in events.read() {
        let load_raw: HashSet<(i32, i32)> =
            get_radius(*cx, *cy, render_distance.0).iter().cloned().collect();

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
            get_radius(cx, cy, render_distance.0).iter().cloned().collect();

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

pub fn get_radius(cx: i32, cy: i32, radius: i32) -> Vec<(i32, i32)> {
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

fn setup_scene(mut commands: Commands) {
    let light_transform = Transform::from_xyz(128., 128., 128.).looking_at(Vec3::new(128., 0., 128.), Vec3::Y);


    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 0.2,
        ..default()
    });

    //commands.spawn((Camera3d::default(), light_transform));
    
    commands.spawn((DirectionalLight {
        illuminance: FULL_DAYLIGHT, 
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


fn debug(
    player_query: Query<(&Player, &Transform, &KinematicCharacterController)>, 
    chunks: Res<RenderedChunks>,
    diagnostics: Res<DiagnosticsStore>,
    mut text_query: Query<&mut Text>,
) {
    let (_, transform, _) = player_query.single().unwrap();
    let x = transform.translation.x;
    let y = transform.translation.y;
    let z = transform.translation.z;

    let mut text = text_query.single_mut().unwrap();
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average()).unwrap_or_default() as usize;

    text.clear();
    text.push_str(&format!("
        X: {x} Y: {y} Z: {z}\n
        FPS: {fps}
        Current chunks: {:?}\n",
    chunks.0.len()));
}
