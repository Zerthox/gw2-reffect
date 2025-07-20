use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    millis: u32,
    min_threshold: u32,
    milli_threshold: u32,
}

impl Time {
    pub const DEFAULT_MIN_THRESHOLD: u32 = 60_000;
    pub const DEFAULT_MILLI_THRESHOLD: u32 = 10_000;
    pub const SEC: u32 = 1000;
    pub const MIN: u32 = 60 * Self::SEC;

    #[inline]
    pub const fn new(mins: u32, secs: u32, millis: u32) -> Self {
        Self::with_threshold(
            mins,
            secs,
            millis,
            Self::DEFAULT_MIN_THRESHOLD,
            Self::DEFAULT_MILLI_THRESHOLD,
        )
    }

    #[inline]
    pub const fn with_threshold(
        mins: u32,
        secs: u32,
        millis: u32,
        min_threshold: u32,
        milli_threshold: u32,
    ) -> Self {
        Self {
            millis: Self::MIN * mins + Self::SEC * secs + millis,
            min_threshold,
            milli_threshold,
        }
    }

    pub fn format(millis: u32, min_threshold: u32, milli_threshold: u32) -> String {
        Self {
            millis,
            min_threshold,
            milli_threshold,
        }
        .to_string()
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            millis,
            min_threshold,
            milli_threshold,
        } = *self;

        if millis >= min_threshold {
            let mins = millis / Self::MIN;
            let secs = (millis % Self::MIN) as f32 / Self::SEC as f32;
            write!(f, "{mins}:{secs:02.0}")
        } else {
            let secs = millis as f32 / Self::SEC as f32;
            let prec = if millis >= milli_threshold { 0 } else { 1 };
            write!(f, "{secs:.prec$}")
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Unit<T>(pub T);

impl<T> Unit<T> {
    const KILO: f32 = 1_000.0;
    const MEGA: f32 = 1_000_000.0;
    const GIGA: f32 = 1_000_000_000.0;

    pub fn format(value: T) -> String
    where
        Self: fmt::Display,
    {
        Self(value).to_string()
    }
}

impl fmt::Display for Unit<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;
        match value {
            Self::GIGA.. => write!(f, "{:.2}B", value / Self::GIGA),
            Self::MEGA.. => write!(f, "{:.2}M", value / Self::MEGA),
            Self::KILO.. => write!(f, "{:.1}k", value / Self::KILO),
            _ => write!(f, "{:.}", (value * 10.0).round_ties_even() / 10.0),
        }
    }
}

impl fmt::Display for Unit<u32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KILO: u32 = Unit::<u32>::KILO as u32;
        const MEGA: u32 = Unit::<u32>::MEGA as u32;
        const GIGA: u32 = Unit::<u32>::GIGA as u32;

        let value = self.0;
        match value {
            GIGA.. => write!(f, "{:.2}B", value as f32 / Self::GIGA),
            MEGA.. => write!(f, "{:.2}M", value as f32 / Self::MEGA),
            KILO.. => write!(f, "{:.1}k", value as f32 / Self::KILO),
            _ => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time() {
        assert_eq!(Time::new(0, 0, 00).to_string(), "0.0");
        assert_eq!(Time::new(0, 0, 1234).to_string(), "1.2");
        assert_eq!(Time::new(3, 4, 567).to_string(), "3:05");
    }

    #[test]
    fn unit_u32() {
        assert_eq!(Unit::format(0), "0");
        assert_eq!(Unit::format(123), "123");
        assert_eq!(Unit::format(1_000), "1.0k");
        assert_eq!(Unit::format(76_590), "76.6k");
        assert_eq!(Unit::format(1_239_000), "1.24M");
    }

    #[test]
    fn unit_f32() {
        assert_eq!(Unit::format(0.0), "0.0");
        assert_eq!(Unit::format(123.49), "123.5");
        assert_eq!(Unit::format(1_000.0), "1.0k");
        assert_eq!(Unit::format(76_590.0), "76.6k");
        assert_eq!(Unit::format(1_239_000.0), "1.24M");
    }
}
