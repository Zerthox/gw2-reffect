use std::sync::Mutex;

// TODO: use future instead?

/// A thread-safe container for a single value entry with an associated retrieval key.
#[derive(Debug)]
pub struct Lockbox<K, V> {
    data: Mutex<Option<(K, V)>>,
}

impl<K, V> Lockbox<K, V>
where
    K: PartialEq,
{
    /// Creates a new lockbox.
    pub const fn new() -> Self {
        Self {
            data: Mutex::new(None),
        }
    }

    /// Writes a new value into the lockbox.
    ///
    /// This will overwrite previous contents.
    /// The value can only be retrieved with an equal key.
    pub fn write(&self, key: K, value: V) {
        *self.data.lock().unwrap() = Some((key, value));
    }

    /// Attempts to retrieve the value, returning [`None`] if the key did not match.
    pub fn try_take(&self, key: K) -> Option<V> {
        let mut guard = self.data.lock().unwrap();
        matches!(&*guard, Some((cur, _)) if cur == &key).then(|| {
            let (_, data) = unsafe { guard.take().unwrap_unchecked() };
            data
        })
    }
}

impl<K, V> Default for Lockbox<K, V>
where
    K: PartialEq,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
