use crate::elements::Pack;
use serde::{Deserialize, Serialize};
use serde_path_to_error::{Error, Track};
use std::{
    borrow::Cow,
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
};
use strum::AsRefStr;

#[derive(Debug, Clone, AsRefStr, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "schema")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Schema<'a> {
    #[serde(alias = "1")]
    V1(Cow<'a, Pack>), // enable borrowed serialization
}

impl<'a> Schema<'a> {
    pub fn latest(pack: &'a Pack) -> Self {
        Self::V1(Cow::Borrowed(pack))
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Option<Self> {
        let path = path.as_ref();
        let file = File::open(path)
            .inspect_err(|err| {
                log::error!("Failed to open pack file \"{}\": {err}", path.display())
            })
            .ok()?;
        let reader = BufReader::new(file);
        let schema = Self::deserialize(reader)
            .inspect_err(|err| {
                let json_err = err.inner();
                log::warn!(
                    "Failed to parse pack file \"{}\": {err} (at {}, line {}, column {})",
                    path.display(),
                    err.path(),
                    json_err.line(),
                    json_err.column(),
                )
            })
            .ok()?;
        log::info!(
            "Added pack \"{}\" from \"{}\" (schema {})",
            schema.name(),
            path.display(),
            schema.as_ref(),
        );
        Some(schema)
    }

    fn deserialize(reader: impl io::Read) -> Result<Self, Error<serde_json::Error>> {
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        let schema: Self = serde_path_to_error::deserialize(&mut deserializer)?;
        deserializer
            .end()
            .map_err(|err| Error::new(Track::new().path(), err))?;
        Ok(schema)
    }

    pub fn save_to_file(&self, file: &File) -> bool {
        let writer = BufWriter::new(file);
        if let Err(err) = serde_json::to_writer_pretty(writer, self) {
            log::error!("Failed to serialize pack \"{}\": {err}", self.name());
            false
        } else {
            true
        }
    }

    pub fn into_pack(self) -> Pack {
        match self {
            Self::V1(pack) => pack.into_owned(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::V1(v1) => &v1.common.name,
        }
    }
}
