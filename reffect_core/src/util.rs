#[macro_export]
macro_rules! non_zero_u32 {
    ( $val:literal ) => {{
        use ::std::num::NonZero;
        const VAL: NonZero<u32> = match NonZero::new($val) {
            Some(val) => val,
            None => panic!(concat!(stringify!($val), " is zero")),
        };
        VAL
    }};
}

pub use non_zero_u32;
