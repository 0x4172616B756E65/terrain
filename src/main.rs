use bevy::{color::palettes::{css::WHITE, tailwind::GREEN_500}, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, log::tracing_subscriber, pbr::light_consts::lux::{FULL_DAYLIGHT}};
use bevy::prelude::*;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, KinematicCharacterController}, render::RapierDebugRenderPlugin};
use terrain::{init::{DebugText, Init}, noise::perlin::Perlin, player::{cursor::CursorPlugin, inventory::inventory::InventoryPlugin, player::{Player, PlayerPlugin}}, simulation::{physics::BallisticsPlugin, sun::DaylightCyclePlugin}, terrain::{chunks::{Chunkbase, RenderDistance, RenderedChunks}, grid::{ChunkRadius, CurrentChunk, GridPlugin}}};
use std::collections::HashSet;



fn main() {
    tracing_subscriber::fmt()
        .init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Init)
        .add_plugins(PlayerPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(BallisticsPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(DaylightCyclePlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default()) 
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CursorPlugin)
        //.add_plugins(PropPlugin)
        .add_systems(Update, debug)
        .run();
}

fn debug(
    player_query: Query<(&Player, &Transform, &KinematicCharacterController)>, 
    chunks: Res<RenderedChunks>,
    diagnostics: Res<DiagnosticsStore>,
    mut text_query: Query<&mut Text, With<DebugText>>,
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
