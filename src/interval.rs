#[derive(Debug, Clone)]
pub struct Interval {
    pub frequency: u32,
    next_update: u32,
}

impl Interval {
    pub fn new(frequency: u32) -> Self {
        Self {
            frequency,
            next_update: 0,
        }
    }

    pub fn triggered(&mut self, now: u32) -> bool {
        if now >= self.next_update {
            self.refresh_next_update(now);
            true
        } else {
            false
        }
    }

    pub fn refresh_next_update(&mut self, now: u32) {
        self.next_update = now + self.frequency;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval() {
        let mut interval = Interval::new(10);

        assert!(interval.triggered(20));
        assert!(!interval.triggered(25));
        assert!(interval.triggered(30));
    }
}
