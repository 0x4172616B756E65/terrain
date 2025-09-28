use bevy::{color::palettes::css::{GRAY, WHITE}, ecs::error::info, log::tracing_subscriber, pbr::light_consts::lux::OVERCAST_DAY};
use bevy::prelude::*;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, KinematicCharacterController}, render::RapierDebugRenderPlugin};
use sysinfo::System;
use terrain::{noise::perlin::Perlin, player::{cursor::CursorPlugin, player::{Player, PlayerPlugin}}, terrain::{chunks::{Chunk, Chunkbase}, grid::{CurrentChunk, GridPlugin}}};

#[derive(Component)]
struct CustomUV;



fn main() {
    tracing_subscriber::fmt().init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CursorPlugin)
        .add_systems(Startup, (setup_scene, init_resources))
        .add_systems(Update, (load_chunks, debug))
        .run();
}



fn init_resources(mut commands: Commands) {
    let perlin = Perlin::new(1, 0.1, 4, 2., 0.5);
    let chunkbase: Chunkbase = Chunkbase::new_with_mesh(32, 32, &perlin, true);

    commands.spawn(Collider::cuboid(1000., 0., 1000.));
    commands.insert_resource(perlin);
    commands.insert_resource(chunkbase);
}

//TODO init perlin as res
fn load_chunks(
    chunkbase: Res<Chunkbase>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<CurrentChunk>, 
    mut commands: Commands
) {
    let stone = materials.add(StandardMaterial {
        base_color: GRAY.into(),
        perceptual_roughness: 0.5,
        ..default()
    });

    let mut mesh_handles = Vec::new();

    for CurrentChunk((chunk_x, chunk_y)) in events.read() {
        for chunk in chunkbase.load_chunks(*chunk_x, *chunk_y, 4) {
            mesh_handles.push(meshes.add(chunk.get_mesh().as_ref().unwrap().clone()));
        }

        for handle in &mesh_handles {
            commands.spawn((
                Mesh3d(handle.clone()),
                MeshMaterial3d(stone.clone()),        
                CustomUV,
            ));
        }
    }
}

fn setup_scene(mut commands: Commands) {
    let light_transform = Transform::from_xyz(128., 64., 128.).looking_at(Vec3::new(128., 0., 128.), Vec3::Y);


    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 0.2,
        ..default()
    });

    //commands.spawn((Camera3d::default(), light_transform));
    
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


fn debug(mut text_query: Query<&mut Text>, player_query: Query<(&Player, &Transform, &KinematicCharacterController)>, mut sys: Local<System>, mut events: EventReader<CurrentChunk>) {
    let (player, transform, controller) = player_query.single().unwrap();
    let x = transform.translation.x;
    let y = transform.translation.y;
    let z = transform.translation.z;

    let mut text = text_query.single_mut().unwrap();
    sys.refresh_memory();
    let used = sys.used_memory() / 1024;

    for CurrentChunk((chunk_x, chunk_y)) in events.read() {
        let chunk = format!("Entered chunk {chunk_x}, {chunk_y}");

    text.clear();
    text.push_str(&format!("
        X: {x} Y: {y} Z: {z}\n
        RAM: {:?}\n
        {:?}\n
        {chunk}",
    used, player.momentum));
    }
}
