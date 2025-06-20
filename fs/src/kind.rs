use std::{
    fs::{self, File},
    io::Read,
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

use zip::ZipArchive;

use crate::{VfsBuffer, VfsError};

#[derive(Debug, Clone)]
pub(crate) enum VfsKind {
    Working,
    ZipArchive(Arc<Mutex<ZipArchive<VfsBuffer>>>),
}

impl VfsKind {
    pub(crate) fn subdirectory_exists(&self, subdirectory: &str) -> bool {
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

    pub(crate) fn open(&self, path: &str) -> Result<VfsBuffer, VfsError> {
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
