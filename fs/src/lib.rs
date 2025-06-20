#![feature(let_chains)]

use std::{
    fs, io::Cursor, ops::Deref, path::Path, sync::{Arc, Mutex}
};

use zip::{result::ZipError, ZipArchive};

#[derive(Debug, Clone)]
enum VirtualFilesystemKind {
    Working,
    ZipArchive(Arc<Mutex<ZipArchive<Cursor<Box<[u8]>>>>>),
}

impl VirtualFilesystemKind {
    fn subdirectory_exists(&self, subdirectory: &str) -> bool {
        match self {
            VirtualFilesystemKind::Working => {
                let path = Path::new(subdirectory);
                let Ok(meta) = fs::metadata(path) else {
                    return false;
                };

                return meta.is_dir();
            }

            VirtualFilesystemKind::ZipArchive(zip) => {
                let prefix = format!("{}/", subdirectory);

                let Ok(mut zip) = zip.lock() else {
                    return false;
                };

                for i in 0..zip.len() {
                    if let Ok(file) = zip.by_index(i) {
                        if file.name().starts_with(&prefix) {
                            return true;
                        }
                    }
                }

                return false;
            }
        }
    }
}

#[derive(Debug)]
pub enum VirtualFilesystemError {
    ZipError(ZipError),
    DirectoryNotFound,
}

impl From<ZipError> for VirtualFilesystemError {
    fn from(value: ZipError) -> Self {
        return Self::ZipError(value);
    }
}

/// A virtual filesystem.
/// It stores the source, and am optional subdirectory.
/// The subdirectory is always delimitered by '/'. It also shouldn't begin or end with '/'.
/// It'll convert to OS-speicifc delimiters before interacting with the real filesystem.
#[derive(Debug, Clone)]
pub struct VirtualFilesystem {
    kind: VirtualFilesystemKind,
    subdirectory: Option<Box<str>>,
}

impl VirtualFilesystem {
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
        kind: VirtualFilesystemKind,
    ) -> Result<Self, VirtualFilesystemError> {
        let subdirectory = subdirectory.map(Self::parse_subdirectory).unwrap_or(None);

        if let Some(dir) = &subdirectory
            && !kind.subdirectory_exists(dir)
        {
            return Err(VirtualFilesystemError::DirectoryNotFound);
        }

        return Ok(Self {
            kind,
            subdirectory,
        });
    }

    pub fn working<S: Deref<Target = str>>(
        subdirectory: Option<S>,
    ) -> Result<Self, VirtualFilesystemError> {
        return Self::from_kind(subdirectory, VirtualFilesystemKind::Working);
    }

    pub fn zip<S: Deref<Target = str>>(
        bytes: &[u8],
        subdirectory: Option<S>,
    ) -> Result<Self, VirtualFilesystemError> {
        return Self::from_kind(
            subdirectory,
            VirtualFilesystemKind::ZipArchive(Arc::new(Mutex::new(ZipArchive::new(Cursor::new(
                bytes.into(),
            ))?))),
        );
    }

    pub fn subdirectory(&self, subdirectory: &str) -> Result<Self, VirtualFilesystemError> {
        let combined = if let Some(dir) = &self.subdirectory {
            format!("{}/{}", dir, subdirectory)
        } else {
            subdirectory.into()
        };

        return Self::from_kind(Some(combined), self.kind.clone());
    }
}
