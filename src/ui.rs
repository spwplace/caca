use crate::ca::RuleMode;

pub struct UiState {
    pub paused: bool,
    pub use_totalistic: bool,
    pub rule_mode: RuleMode,
    pub sparse_density: f32,
    pub speed_divisor: u64,
    pub should_randomize: bool,
    pub totalistic_changed: bool,
    pub rule_mode_changed: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            paused: false,
            use_totalistic: true,
            rule_mode: RuleMode::Classic,
            sparse_density: 0.1,
            speed_divisor: 1,
            should_randomize: false,
            totalistic_changed: false,
            rule_mode_changed: false,
        }
    }
}
