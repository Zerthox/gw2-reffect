use std::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]

pub struct Time(pub u32);

impl Time {
    pub const SEC: u32 = 1000;
    pub const MIN: u32 = 60 * Self::SEC;

    #[allow(unused)]
    pub const fn new(min: u32, sec: u32, ms: u32) -> Self {
        Self(Self::MIN * min + Self::SEC * sec + ms)
    }

    pub fn format(value: u32) -> String {
        Self(value).to_string()
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;
        let min = value / Self::MIN;
        let secs = (value % Self::MIN) as f32 / Self::SEC as f32;
        if min > 0 {
            write!(f, "{min}:{secs:0>4.1}")
        } else {
            write!(f, "{secs:.1}")
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

    pub fn format_if(value: T, unit: bool) -> String
    where
        T: fmt::Display,
        Self: fmt::Display,
    {
        if unit {
            Self::format(value)
        } else {
            value.to_string()
        }
    }
}

impl fmt::Display for Unit<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;
        match value {
            Self::GIGA.. => write!(f, "{:.2}B", value / Self::GIGA),
            Self::MEGA.. => write!(f, "{:.2}M", value / Self::MEGA),
            Self::KILO.. => write!(f, "{:.1}k", value / Self::KILO),
            _ => write!(f, "{value:.1}"),
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
        assert_eq!(Time::format(0), "0.0");
        assert_eq!(Time::format(1234), "1.2");
        assert_eq!(Time::new(3, 4, 567).to_string(), "3:04.6");
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
