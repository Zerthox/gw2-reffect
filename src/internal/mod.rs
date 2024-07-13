mod error;
mod shared;

pub use self::{error::*, shared::Buff};

use self::shared::BuffsResult;
use dlopen2::wrapper::{Container, WrapperApi};
use nexus::paths::get_game_dir;
use std::{fmt, fs, path::PathBuf, slice};

static DLL: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "./deps/reffect_internal.dll"
));

#[derive(Debug, WrapperApi)]
pub struct Exports {
    update_buffs: extern "C-unwind" fn() -> BuffsResult,
}

pub struct Internal(Result<Container<Exports>, Error>);

impl Internal {
    pub fn load() -> Self {
        Self(Self::load_dll())
    }

    pub fn unload(&mut self) {
        self.0 = Err(Error::Extract);

        if let Some(path) = Self::path() {
            if let Err(err) = fs::remove_file(&path) {
                log::warn!("Failed to remove internal: {err}");
            } else {
                log::debug!("Removed internal from \"{}\"", path.display());
            }
        }
    }

    fn path() -> Option<PathBuf> {
        get_game_dir().map(|dir| dir.join("reffect_internal.dll"))
    }

    fn load_dll() -> Result<Container<Exports>, Error> {
        let path = Self::path().ok_or(Error::Extract)?;
        log::debug!("Extracting internal to \"{}\"", path.display());

        fs::write(&path, DLL).map_err(|err| {
            log::error!("Failed to extract internal: {err}");
            Error::Extract
        })?;

        unsafe { Container::load(path) }.map_err(|err| {
            log::error!("Failed to load internal: {err}");
            Error::Load
        })
    }

    /// Returns the internal exports.
    pub fn exports(&self) -> Result<&Container<Exports>, Error> {
        self.0.as_ref().map_err(|err| *err)
    }

    /// Returns the current buffs or an error.
    pub fn update_buffs(&mut self) -> Result<&[Buff], Error> {
        let exports = self.exports()?;
        let result = exports.update_buffs();
        if result.error == shared::Error::None {
            Ok(unsafe { slice::from_raw_parts(result.buffs, result.len) })
        } else {
            Err(result.error.into())
        }
    }
}

impl fmt::Debug for Internal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0
            .as_ref()
            .map(|container| unsafe { container.into_raw() })
            .fmt(f)
    }
}
