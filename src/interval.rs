#[derive(Debug, Clone)]
pub struct Interval {
    pub frequency: f64,
    pub last_update: f64,
}

impl Interval {
    pub fn new(frequency: f64) -> Self {
        Self {
            frequency,
            last_update: 0.0,
        }
    }

    pub fn triggered(&mut self, time: f64) -> bool {
        let passed = time - self.last_update;
        if passed >= self.frequency {
            self.last_update = time;
            true
        } else {
            false
        }
    }
}
