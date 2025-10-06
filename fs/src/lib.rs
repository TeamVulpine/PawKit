use std::{
    io,
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

use zip::{ZipArchive, result::ZipError};

mod buffer;
pub use buffer::*;

mod list_directories;
pub use list_directories::*;

mod list_files;
pub use list_files::*;

mod list_files_recursive;
pub use list_files_recursive::*;

mod kind;
use kind::*;

#[derive(Debug)]
pub enum VfsError {
    ZipError(ZipError),
    IoError(io::Error),
    NotFound,
    Other,
}

impl From<io::Error> for VfsError {
    fn from(value: io::Error) -> Self {
        return Self::IoError(value);
    }
}

impl From<ZipError> for VfsError {
    fn from(value: ZipError) -> Self {
        return Self::ZipError(value);
    }
}

/// A virtual filesystem.
/// It stores the source, and am optional subdirectory.
/// The subdirectory is always delimitered by '/'. It also shouldn't begin or end with '/'.
/// It'll convert to OS-speicifc delimiters before interacting with the real filesystem.
#[derive(Debug, Clone)]
pub struct Vfs {
    kind: VfsKind,
    subdirectory: Option<Box<str>>,
}

impl Vfs {
    fn parse_subdirectory<S: Deref<Target = str>>(subdirectory: S) -> Option<Box<str>> {
        let mut data = subdirectory.split('/').collect::<Vec<_>>();

        data.retain(|it| !it.is_empty() && *it != ".");

        let mut dirrectory = vec![];

        for value in data {
            if value == ".." {
                dirrectory.pop();
            } else {
                dirrectory.push(value);
            }
        }

        if dirrectory.is_empty() {
            return None;
        }

        return Some(dirrectory.join("/").into_boxed_str());
    }

    fn from_kind<S: Deref<Target = str>>(
        subdirectory: Option<S>,
        kind: VfsKind,
    ) -> Result<Self, VfsError> {
        let subdirectory = subdirectory.map(Self::parse_subdirectory).unwrap_or(None);

        if let Some(dir) = &subdirectory
            && !kind.subdirectory_exists(dir)
        {
            return Err(VfsError::NotFound);
        }

        return Ok(Self { kind, subdirectory });
    }

    pub fn working() -> Result<Self, VfsError> {
        return Self::from_kind::<&str>(None, VfsKind::Working);
    }

    pub fn zip<B: Into<VfsBuffer>>(buf: B) -> Result<Self, VfsError> {
        return Self::from_kind::<&str>(
            None,
            VfsKind::ZipArchive(Arc::new(Mutex::new(ZipArchive::new(buf.into())?))),
        );
    }

    pub fn subdirectory(&self, subdirectory: &str) -> Result<Self, VfsError> {
        let combined = if let Some(dir) = &self.subdirectory {
            format!("{}/{}", dir, subdirectory)
        } else {
            subdirectory.into()
        };

        return Self::from_kind(Some(combined), self.kind.clone());
    }

    pub fn open(&self, path: &str) -> Result<VfsBuffer, VfsError> {
        let path = self
            .subdirectory
            .as_ref()
            .map(|it| format!("{}/{}", it, path))
            .unwrap_or_else(|| path.into());

        return self.kind.open(&path);
    }

    pub fn list_subdirectories(&self) -> Result<VfsListDirectories, VfsError> {
        match &self.kind {
            VfsKind::Working => {
                let path = self
                    .subdirectory
                    .as_ref()
                    .map_or(Path::new("."), |it| Path::new(it.deref()));

                let entries = path.read_dir()?;

                return Ok(VfsListDirectories::Working(entries));
            }

            VfsKind::ZipArchive(zip) => {
                return Ok(VfsListDirectories::ZipArchive {
                    index: 0,
                    prefix: self
                        .subdirectory
                        .clone()
                        .map_or("".into(), |it| format!("{}/", it).into()),
                    zip: zip.clone(),
                    seen: std::collections::HashSet::new(),
                });
            }
        };
    }

    pub fn list_files(&self) -> Result<VfsListFiles, VfsError> {
        match &self.kind {
            VfsKind::Working => {
                let path = self
                    .subdirectory
                    .as_ref()
                    .map_or(Path::new("."), |it| Path::new(it.deref()));

                let entries = path.read_dir()?;

                return Ok(VfsListFiles::Working(entries));
            }

            VfsKind::ZipArchive(zip) => {
                return Ok(VfsListFiles::ZipArchive {
                    index: 0,
                    prefix: self
                        .subdirectory
                        .clone()
                        .map_or("".into(), |it| format!("{}/", it).into()),
                    zip: zip.clone(),
                });
            }
        };
    }

    pub fn list_files_recursive(&self) -> Result<VfsListFilesRecursive, VfsError> {
        return Ok(VfsListFilesRecursive {
            stack: vec![(self.clone(), self.list_subdirectories()?)],
            files: Some(self.list_files()?),
            prefix: self.subdirectory.clone().unwrap_or_default(),
        });
    }
}

pub trait VfsListUtils: Iterator<Item = Result<String, VfsError>> + Sized {
    fn with_extension<S: Deref<Target = str>>(
        self,
        ext: S,
    ) -> impl Iterator<Item = Result<String, VfsError>> {
        let ext = ext.to_string();
        return self.filter(move |it| it.as_ref().map(|it| it.ends_with(&ext)).unwrap_or(true));
    }
}

impl<T: Iterator<Item = Result<String, VfsError>> + Sized> VfsListUtils for T {}
