#![feature(let_chains)]

use std::{
    collections::HashSet,
    fs::{self, File, ReadDir},
    io::{self, Read},
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

use zip::{result::ZipError, ZipArchive};

use crate::buffer::VfsBuffer;

pub mod buffer;

#[derive(Debug, Clone)]
enum VfsKind {
    Working,
    ZipArchive(Arc<Mutex<ZipArchive<VfsBuffer>>>),
}

impl VfsKind {
    fn subdirectory_exists(&self, subdirectory: &str) -> bool {
        match self {
            VfsKind::Working => {
                let path = Path::new(subdirectory);
                let Ok(meta) = fs::metadata(path) else {
                    return false;
                };

                return meta.is_dir();
            }

            VfsKind::ZipArchive(zip) => {
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

    fn open(&self, path: &str) -> Result<VfsBuffer, VfsError> {
        match self {
            VfsKind::Working => {
                let path = Path::new(path);
                return Ok(File::open(path)?.into());
            }

            VfsKind::ZipArchive(zip) => {
                let Ok(mut zip) = zip.lock() else {
                    return Err(VfsError::Other);
                };

                let mut file = zip.by_name(path)?;

                let mut buf = vec![];
                file.read_to_end(&mut buf)?;

                return Ok(buf.deref().into());
            }
        }
    }
}

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

pub enum VfsListDirectories {
    Working(ReadDir),
    ZipArchive {
        index: usize,
        prefix: Box<str>,
        zip: Arc<Mutex<ZipArchive<VfsBuffer>>>,
        seen: HashSet<String>,
    },
}

impl Iterator for VfsListDirectories {
    type Item = Result<String, VfsError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            VfsListDirectories::Working(iter) => {
                let mut name = None;
                while let None = name {
                    match iter.next()?.map_err(Into::into) {
                        Ok(dir) => match dir.file_type().map_err(Into::into) {
                            Ok(it) => {
                                if it.is_dir() {
                                    name = dir.file_name().to_str().map(Into::into);
                                }
                            }
                            Err(err) => return Some(Err(err)),
                        },

                        Err(err) => {
                            return Some(Err(err));
                        }
                    }
                }

                return Some(Ok(name?));
            }

            VfsListDirectories::ZipArchive {
                index,
                prefix,
                zip,
                seen,
            } => {
                let Ok(mut zip) = zip.lock() else {
                    return Some(Err(VfsError::Other));
                };

                while *index < zip.len() {
                    let file = match zip.by_index(*index).map_err(Into::into) {
                        Ok(file) => file,
                        Err(err) => return Some(Err(err)),
                    };
                    *index += 1;

                    let name = file.name();
                    let prefix: &str = &prefix;
                    if name.starts_with(prefix) {
                        let remaining = &name[prefix.len()..];

                        if let Some((dir, _)) = remaining.split_once('/') {
                            if seen.insert(dir.to_string()) {
                                return Some(Ok(dir.to_string()));
                            }
                        }
                    }
                }

                return None;
            }
        }
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

    pub fn working<S: Deref<Target = str>>(subdirectory: S) -> Result<Self, VfsError> {
        return Self::from_kind(Some(subdirectory), VfsKind::Working);
    }

    pub fn zip<B: Into<VfsBuffer>, S: Deref<Target = str>>(
        bytes: B,
        subdirectory: Option<S>,
    ) -> Result<Self, VfsError> {
        return Self::from_kind(
            subdirectory,
            VfsKind::ZipArchive(Arc::new(Mutex::new(ZipArchive::new(bytes.into())?))),
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
        return self.kind.open(path);
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
                })
            }
        };
    }
}
