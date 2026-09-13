use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Time {
    /// Minutes and seconds.
    Minutes { mins: u32, secs: u32 },

    /// Seconds.
    Seconds { secs: u32 },

    /// Seconds with hundred milliseconds.
    Millis { secs: u32, hundreds: u32 },
}

impl Time {
    /// Default threshold to display minutes.
    pub const DEFAULT_MIN_THRESHOLD: u32 = 60_000;

    /// Default threshold to display milliseconds.
    pub const DEFAULT_MILLI_THRESHOLD: u32 = 10_000;

    /// Milliseconds in a second.
    pub const SEC: u32 = 1000;

    /// Milliseconds in a minute.
    pub const MIN: u32 = 60 * Self::SEC;

    #[inline]
    pub const fn new(mins: u32, secs: u32, millis: u32) -> Self {
        Self::new_with_threshold(
            mins,
            secs,
            millis,
            Self::DEFAULT_MIN_THRESHOLD,
            Self::DEFAULT_MILLI_THRESHOLD,
        )
    }

    #[inline]
    pub const fn new_with_threshold(
        mins: u32,
        secs: u32,
        millis: u32,
        min_threshold: u32,
        milli_threshold: u32,
    ) -> Self {
        Self::from_millis_with_threshold(
            Self::MIN * mins + Self::SEC * secs + millis,
            min_threshold,
            milli_threshold,
        )
    }

    #[inline]
    pub const fn from_millis(millis: u32) -> Self {
        Self::from_millis_with_threshold(
            millis,
            Self::DEFAULT_MIN_THRESHOLD,
            Self::DEFAULT_MILLI_THRESHOLD,
        )
    }

    #[inline]
    pub const fn from_millis_with_threshold(
        millis: u32,
        min_threshold: u32,
        milli_threshold: u32,
    ) -> Self {
        let ceil_secs = millis.saturating_add(Self::SEC - 1);
        let ceil_hundreds = millis.saturating_add(99);
        if min_threshold > 0 && ceil_secs >= min_threshold {
            Self::Minutes {
                mins: ceil_secs / Self::MIN,
                secs: (ceil_secs % Self::MIN) / Self::SEC,
            }
        } else if ceil_hundreds >= milli_threshold {
            Self::Seconds {
                secs: ceil_secs / Self::SEC,
            }
        } else {
            Self::Millis {
                secs: ceil_hundreds / Self::SEC,
                hundreds: (ceil_hundreds % Self::SEC) / 100,
            }
        }
    }

    #[inline]
    pub fn format_with_threshold(millis: u32, min_threshold: u32, milli_threshold: u32) -> String {
        Self::from_millis_with_threshold(millis, min_threshold, milli_threshold).to_string()
    }
}

impl fmt::Display for Time {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Minutes { mins, secs } => write!(f, "{mins}:{secs:02}"),
            Self::Seconds { secs } => write!(f, "{secs}"),
            Self::Millis { secs, hundreds } => write!(f, "{secs}.{hundreds}"),
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

    #[inline]
    pub fn format(value: T) -> String
    where
        Self: fmt::Display,
    {
        Self(value).to_string()
    }
}

impl fmt::Display for Unit<f32> {
    #[inline]
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
    #[inline]
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
        assert_eq!(Time::format_with_threshold(0, 0, 0), "0");
        assert_eq!(Time::new(0, 0, 0).to_string(), "0.0");
        assert_eq!(Time::new(0, 0, 1).to_string(), "0.1");
        assert_eq!(Time::new(0, 0, 99).to_string(), "0.1");
        assert_eq!(Time::new(0, 0, 100).to_string(), "0.1");
        assert_eq!(Time::new(0, 1, 234).to_string(), "1.3");
        assert_eq!(Time::new(0, 9, 900).to_string(), "9.9");
        assert_eq!(Time::new(0, 9, 901).to_string(), "10");
        assert_eq!(Time::new(0, 9, 999).to_string(), "10");
        assert_eq!(Time::new(0, 10, 0).to_string(), "10");
        assert_eq!(Time::new(0, 10, 1).to_string(), "11");
        assert_eq!(Time::new(0, 59, 0).to_string(), "59");
        assert_eq!(Time::new(0, 59, 1).to_string(), "1:00");
        assert_eq!(Time::new(0, 59, 999).to_string(), "1:00");
        assert_eq!(Time::new(0, 60, 0).to_string(), "1:00");
        assert_eq!(Time::new(0, 60, 1).to_string(), "1:01");
        assert_eq!(Time::new(3, 4, 564).to_string(), "3:05");
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
