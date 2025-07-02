use serde::{Deserialize, Deserializer, Serialize, de};
use std::{any::type_name, marker::PhantomData, ops};

/// Helper to migrate data from an old serde format via `deserialize_with`.
pub fn migrate<'de, D, T, P>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
    P: Deserialize<'de> + Into<T>,
{
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Value<T, P> {
        Current(T),
        Previous(P),
    }

    Value::<T, P>::deserialize(deserializer)
        .map(|value| match value {
            Value::Current(inner) => inner,
            Value::Previous(prev) => {
                log::debug!("Migrating {} to {}", type_name::<P>(), type_name::<T>());
                prev.into()
            }
        })
        .map_err(|_| {
            <D::Error as de::Error>::custom(format!("failed to migrate {}", type_name::<T>()))
        })
}

/// Wrapper type to migrate data from an old serde format.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Migrate<T, P> {
    pub inner: T,
    _prev: PhantomData<P>,
}

impl<T, P> Migrate<T, P> {
    pub fn new(inner: impl Into<T>) -> Self {
        Self {
            inner: inner.into(),
            _prev: PhantomData,
        }
    }
}

impl<T, P> ops::Deref for Migrate<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, P> ops::DerefMut for Migrate<T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, P> From<T> for Migrate<T, P> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T, P> Serialize for Migrate<T, P>
where
    T: From<P> + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T, P> Deserialize<'de> for Migrate<T, P>
where
    T: From<P> + for<'a> Deserialize<'a>,
    for<'a> P: Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        migrate::<'de, D, T, P>(deserializer).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Deserialize)]
    struct A(u32);

    #[derive(Debug, PartialEq, Deserialize)]
    struct B(f32);

    impl From<B> for A {
        fn from(value: B) -> Self {
            Self((1000.0 * value.0) as u32)
        }
    }

    #[test]
    fn migrate_attr() {
        #[derive(Debug, Deserialize)]
        struct Parent {
            #[serde(deserialize_with = "migrate::<_, _, B>")]
            pub field: A,
        }

        let result = serde_json::from_str::<Parent>("{ \"field\": 123.456 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().field, A(123456));

        let result = serde_json::from_str::<Parent>("{ \"field\": 123 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().field, A(123));
    }

    #[test]
    fn migrate_wrapper() {
        let result = serde_json::from_str::<Migrate<A, B>>("123.456");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().inner, A(123456));

        let result = serde_json::from_str::<Migrate<A, B>>("123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().inner, A(123));
    }
}
