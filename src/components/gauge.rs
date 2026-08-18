#[derive(Clone, Copy, Debug)]
pub struct GaugeData {
    pub gross_wpm: f32,
    pub net_wpm: f32,
    pub accuracy: f32,
    pub errors: usize,
    pub elapsed_secs: f32,
    pub progress: f32, // 0.0 to 1.0
    pub target_wpm: u32,
}

impl Default for GaugeData {
    fn default() -> Self {
        Self {
            gross_wpm: 0.0,
            net_wpm: 0.0,
            accuracy: 100.0,
            errors: 0,
            elapsed_secs: 0.0,
            progress: 0.0,
            target_wpm: 35,
        }
    }
}
