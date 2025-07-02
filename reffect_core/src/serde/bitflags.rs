use enumflags2::{BitFlag, BitFlags};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S, T>(flags: &BitFlags<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + BitFlag,
{
    let vec = flags.into_iter().collect::<Vec<T>>();
    vec.serialize(serializer)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<BitFlags<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + BitFlag,
{
    let flags = Vec::<T>::deserialize(deserializer)?;
    Ok(BitFlags::from_iter(flags))
}
