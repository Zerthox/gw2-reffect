use std::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Pretty<T>(pub T);

impl<T> Pretty<T> {
    const KILO: f32 = 1_000.0;

    const MEGA: f32 = 1_000_000.0;

    const KILO_MIN: f32 = 10_000.0;

    const MEGA_MIN: f32 = 1_000_000.0;

    pub fn string_if(value: T, pretty: bool) -> String
    where
        T: fmt::Display,
        Pretty<T>: fmt::Display,
    {
        if pretty {
            Self(value).to_string()
        } else {
            value.to_string()
        }
    }
}

impl fmt::Display for Pretty<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;
        match value {
            Self::MEGA_MIN.. => write!(f, "{:.1}M", value / Self::MEGA),
            Self::KILO_MIN.. => write!(f, "{:.1}k", value / Self::KILO),
            _ => write!(f, "{value:.1}"),
        }
    }
}

impl fmt::Display for Pretty<u32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const MEGA_MIN: u32 = Pretty::<u32>::MEGA_MIN as u32;
        const KILO_MIN: u32 = Pretty::<u32>::KILO_MIN as u32;

        let value = self.0;
        match value {
            MEGA_MIN.. => write!(f, "{:.1}M", value as f32 / Self::MEGA),
            KILO_MIN.. => write!(f, "{:.1}k", value as f32 / Self::KILO),
            _ => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32() {
        assert_eq!(Pretty(0).to_string(), "0");
        assert_eq!(Pretty(123).to_string(), "123");
        assert_eq!(Pretty(10_000).to_string(), "10.0k");
        assert_eq!(Pretty(76_590).to_string(), "76.6k");
        assert_eq!(Pretty(1_390_000).to_string(), "1.4M");
    }

    #[test]
    fn f32() {
        assert_eq!(Pretty(0.0).to_string(), "0.0");
        assert_eq!(Pretty(123.49).to_string(), "123.5");
        assert_eq!(Pretty(10_000.0).to_string(), "10.0k");
        assert_eq!(Pretty(76_590.0).to_string(), "76.6k");
        assert_eq!(Pretty(1_390_000.0).to_string(), "1.4M");
    }
}
