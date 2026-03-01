use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

pub struct TempFile {
    file: File,
    path: PathBuf,
}

impl TempFile {
    const EXT: &str = "tmp";

    #[inline]
    pub fn create(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into().with_added_extension(Self::EXT);
        Ok(Self {
            file: File::create(&path)?,
            path,
        })
    }

    #[inline]
    pub fn target_path(&self) -> PathBuf {
        self.path.with_extension("")
    }

    #[inline]
    pub fn file(&self) -> &File {
        &self.file
    }

    #[inline]
    pub fn persist(self) -> io::Result<()> {
        let target = self.target_path();
        let Self { file, path } = self;
        file.sync_all()?;
        drop(file);
        fs::rename(path, target)
    }

    #[inline]
    pub fn is_temp(path: impl AsRef<Path>) -> bool {
        matches!(path.as_ref().extension(), Some(ext) if ext == Self::EXT)
    }
}
