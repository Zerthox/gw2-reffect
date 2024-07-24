use crate::{elements::Pack, util::file_name};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};
use strum::AsRefStr;

#[derive(Debug, Clone, AsRefStr, Serialize, Deserialize)]
#[serde(tag = "schema")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Schema<'a> {
    #[serde(alias = "1")]
    V1(Cow<'a, Pack>), // enable borrowed serialization

    #[serde(untagged)]
    Unknown(Pack),
}

impl<'a> Schema<'a> {
    pub fn latest(pack: &'a Pack) -> Self {
        Self::V1(Cow::Borrowed(pack))
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Option<Self> {
        let path = path.as_ref();
        let file = File::open(path)
            .inspect_err(|err| {
                log::error!("Failed to open pack file \"{}\": {err}", file_name(path))
            })
            .ok()?;
        let reader = BufReader::new(file);
        let schema = serde_json::from_reader::<_, Self>(reader)
            .inspect_err(|err| {
                log::warn!(
                    "Failed to parse pack file \"{}\": {err} (line {}, column {})",
                    file_name(path),
                    err.line(),
                    err.column(),
                )
            })
            .ok()?;
        log::info!(
            "Added pack \"{}\" from \"{}\" (schema {})",
            schema.name(),
            file_name(path),
            schema.as_ref(),
        );
        Some(schema)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        match File::create(path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                if let Err(err) = serde_json::to_writer_pretty(writer, self) {
                    log::error!(
                        "Failed to serialize pack \"{}\" to \"{}\": {err}",
                        self.name(),
                        file_name(path)
                    );
                }
                true
            }
            Err(err) => {
                log::error!(
                    "Failed to save pack \"{}\" to \"{}\": {err}",
                    self.name(),
                    file_name(path)
                );
                false
            }
        }
    }

    pub fn into_pack(self) -> Pack {
        match self {
            Self::V1(pack) => pack.into_owned(),
            Self::Unknown(pack) => {
                log::info!("Converting unknown pack schema to v1");
                pack
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::V1(v1) => &v1.common.name,
            Self::Unknown(v1) => &v1.common.name,
        }
    }
}
