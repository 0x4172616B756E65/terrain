pub struct PlayerState {
    pub is_pressing_movement_key: bool,
    pub debug_flying: bool
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { is_pressing_movement_key: false, debug_flying: false }
    }
}
