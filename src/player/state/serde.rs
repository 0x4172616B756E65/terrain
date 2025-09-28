use std::{env, fs::File, io::{BufReader, Write}, path::PathBuf};

use bevy::ecs::system::Query;

use crate::player::{player::Player, state::player_state::PlayerState};

fn read_player_state() -> PlayerState {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
        path.push(".local/share/player_config.json");

    let file = File::open(path).unwrap();
    let file_reader = BufReader::new(file);

    serde_json::from_reader(file_reader).unwrap()
}

fn write_player_state(player_query: Query<&Player>) {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
        path.push(".local/share/player_config.json");

    let player_state: &PlayerState = &player_query.single().unwrap().state;
    let json = serde_json::to_string_pretty(player_state).unwrap();
    let mut file = File::create(path).unwrap();

    file.write_all(json.as_bytes()).unwrap();
}
