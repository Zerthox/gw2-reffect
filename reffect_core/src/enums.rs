use strum::{EnumCount, VariantArray};

pub const fn check_variant_array<T>()
where
    T: VariantArray + EnumCount,
{
    if T::VARIANTS.len() != T::COUNT {
        panic!("mismatch between VariantArray and EnumCount");
    }
}

pub trait EnumStaticVariants: Sized {
    fn with_variants<R>(action: impl FnOnce(&[Self]) -> R) -> R;
}

impl<T> EnumStaticVariants for T
where
    T: VariantArray,
{
    fn with_variants<R>(action: impl FnOnce(&[Self]) -> R) -> R {
        action(Self::VARIANTS)
    }
}

/// Helper to implement [`EnumStaticVariants`] for enums already implementing [`IntoEnumIterator`].
#[macro_export]
macro_rules! impl_static_variants {
    ($ty:ty) => {
        impl $crate::enums::EnumStaticVariants for $ty {
            fn with_variants<R>(action: impl FnOnce(&[Self]) -> R) -> R {
                use ::std::{thread_local, vec::Vec};
                use ::strum::IntoEnumIterator;

                thread_local! { static VARIANTS: Vec<$ty> = <$ty as IntoEnumIterator>::iter().collect(); };
                VARIANTS.with(|variants| action(variants))
            }
        }
    };
}

pub use impl_static_variants;
