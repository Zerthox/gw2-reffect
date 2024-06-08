#[derive(Debug, Clone)]
pub struct Interval {
    pub frequency: f64,
    pub next_update: f64,
}

impl Interval {
    pub fn new(frequency: f64) -> Self {
        Self {
            frequency,
            next_update: 0.0,
        }
    }

    pub fn triggered(&mut self, time: f64) -> bool {
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
        let mut interval = Interval::new(1.0);

        assert!(interval.triggered(2.0));
        assert!(!interval.triggered(2.5));
        assert!(interval.triggered(3.0));
    }
}
