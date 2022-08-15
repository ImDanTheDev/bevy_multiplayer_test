pub struct MultiplayerConfig {
    pub tps: u32,
}

impl MultiplayerConfig {
    pub fn new(tps: u32) -> Self {
        Self { tps }
    }
}
