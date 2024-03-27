#[derive(Debug, Clone)]
pub struct Interval {
    pub frequency: u32,
    pub last_update: u32,
}

impl Interval {
    pub fn new(frequency: u32) -> Self {
        Self {
            frequency,
            last_update: 0,
        }
    }

    pub fn triggered(&mut self, tick: u32) -> bool {
        let passed = tick - self.last_update;
        if passed >= self.frequency {
            self.last_update = tick;
            true
        } else {
            false
        }
    }
}
