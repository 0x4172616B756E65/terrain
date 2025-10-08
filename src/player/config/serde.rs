use std::{env, fs::File, io::{BufReader, Write}, path::PathBuf};

use bevy::ecs::system::Query;

use crate::player::{player::Player, config::player_config::PlayerConfig};

fn _read_player_state() -> PlayerConfig {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
        path.push(".local/share/player_config.json");

    let file = File::open(path).unwrap();
    let file_reader = BufReader::new(file);

    serde_json::from_reader(file_reader).unwrap()
}

fn _write_player_state(player_query: Query<&Player>) {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
        path.push(".local/share/player_config.json");

    let player_state: &PlayerConfig = &player_query.single().unwrap().config;
    let json = serde_json::to_string_pretty(player_state).unwrap();
    let mut file = File::create(path).unwrap();

    file.write_all(json.as_bytes()).unwrap();
}
