use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct Interval {
    pub frequency: u32,
    next_update: u32,
}

impl Interval {
    #[inline]
    pub const fn new(frequency: u32) -> Self {
        Self {
            frequency,
            next_update: 0,
        }
    }

    #[inline]
    pub fn triggered(&mut self, now: u32) -> bool {
        if now >= self.next_update {
            self.refresh_next_update(now);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn refresh_next_update(&mut self, now: u32) {
        self.next_update = now + self.frequency;
    }
}

#[derive(Debug)]
pub struct AtomicInterval {
    pub frequency: u32,
    next_update: AtomicU32,
}

impl AtomicInterval {
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
        let mut interval = Interval::new(10);

        assert!(interval.triggered(20));
        assert!(!interval.triggered(25));
        assert!(interval.triggered(30));
    }

    #[test]
    fn atomic_interval() {
        let interval = AtomicInterval::new(10);

        assert!(interval.triggered(20));
        assert!(!interval.triggered(25));
        assert!(interval.triggered(30));
    }
}
