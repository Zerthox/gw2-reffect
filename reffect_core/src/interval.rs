use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct Interval {
    pub frequency: u32,
    next_update: AtomicU32,
}

impl Interval {
    #[inline]
    pub const fn new(frequency: u32) -> Self {
        Self {
            frequency,
            next_update: AtomicU32::new(0),
        }
    }

    #[inline]
    pub fn triggered(&self, now: u32) -> bool {
        if now >= self.next_update.load(Ordering::Relaxed) {
            self.refresh_next_update(now);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn refresh_next_update(&self, now: u32) {
        self.next_update
            .store(now + self.frequency, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval() {
        let interval = Interval::new(10);

        assert!(interval.triggered(20));
        assert!(!interval.triggered(25));
        assert!(interval.triggered(30));
    }
}
