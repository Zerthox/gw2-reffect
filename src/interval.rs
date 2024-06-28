#[derive(Debug, Clone)]
pub struct Interval {
    pub frequency: u32,
    pub next_update: u32,
}

impl Interval {
    pub fn new(frequency: u32) -> Self {
        Self {
            frequency,
            next_update: 0,
        }
    }

    pub fn triggered(&mut self, time: u32) -> bool {
        if time >= self.next_update {
            self.next_update = time + self.frequency;
            true
        } else {
            false
        }
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
