pub struct UiState {
    pub paused: bool,
    pub use_totalistic: bool,
    pub speed_divisor: u64,
    pub should_randomize: bool,
    pub totalistic_changed: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            paused: false,
            use_totalistic: false,
            speed_divisor: 1,
            should_randomize: false,
            totalistic_changed: false,
        }
    }
}
