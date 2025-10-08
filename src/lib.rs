pub mod init;

pub mod noise {
    pub mod perlin;
    pub mod perlin_cpu;
    pub mod poisson_disc;
}

pub mod player {
    pub mod inventory {
        pub mod hud;
        pub mod inventory;
        pub mod items;
    }
    pub mod config {
        pub mod serde;
        pub mod player_config;
        pub mod serde_keyboard;
        pub mod serde_mouse;
    }
    pub mod cursor;
    pub mod player;
    pub mod player_input;
    pub mod player_attack;
    pub mod camera_controller;
}

pub mod terrain {
    /*pub mod props {
        pub mod props;
        pub mod trees {
            pub mod tree;
        }
    }*/
    pub mod grid;
    pub mod chunks;
    pub mod collision;
}

pub mod simulation {
    pub mod sun;
    pub mod physics;
    pub mod ballistics {
        pub mod ammunition;
    }
}
