pub mod noise {
    pub mod perlin;
}

pub mod player {
    pub mod state {
        pub mod serde;
        pub mod player_state;
        pub mod serde_keyboard;
        pub mod serde_mouse;
    }
    pub mod camera;
    pub mod cursor;
    pub mod player;
    pub mod player_input;
}

pub mod terrain {
    pub mod grid;
    pub mod chunks;
    pub mod collision;
}
