pub struct ApplicationContext {
    /// Total time in seconds that the application has been running
    total_s: f64,
    /// Delta time since last frame
    delta_s: f64,
}

impl ApplicationContext {
    pub fn new() -> ApplicationContext {
        ApplicationContext {
            total_s: 0.0,
            delta_s: 0.0,
        }
    }

    pub(super) fn pre_tick(&mut self, delta_s: f64) {
        self.delta_s = delta_s;
        self.total_s += delta_s;
    }

    pub fn total_s(&self) -> f64 {
        self.total_s
    }

    pub fn delta_s(&self) -> f64 {
        self.delta_s
    }
}
