use bevy::prelude::*;

use crate::player::player::Player;

pub struct GridPlugin;

#[derive(Resource, Default)]
struct LastChunk((i32, i32));

#[derive(Event)]
pub struct CurrentChunk(pub (i32, i32));

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LastChunk>()
            .add_event::<CurrentChunk>()
            .add_systems(Update, enter_chunk_event)
        ;
    }
}



fn enter_chunk_event(
    mut last_chunk: ResMut<LastChunk>,
    player_query: Query<&Transform, With<Player>>,
    mut events: EventWriter<CurrentChunk>
) {
    let transform = player_query.single().unwrap();
    let current_chunk = (
        (transform.translation.x / 32.0).floor() as i32,
        (transform.translation.z / 32.0).floor() as i32,
    );

    if current_chunk != last_chunk.0 {
        events.send(CurrentChunk(current_chunk));
        last_chunk.0 = current_chunk;
    }
}
