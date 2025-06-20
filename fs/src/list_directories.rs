use std::{
    collections::HashSet,
    fs::ReadDir,
    sync::{Arc, Mutex},
};

use zip::ZipArchive;

use crate::{buffer::VfsBuffer, VfsError};

pub enum VfsListDirectories {
    Working(ReadDir),
    ZipArchive {
        index: usize,
        prefix: Box<str>,
        zip: Arc<Mutex<ZipArchive<VfsBuffer>>>,
        seen: HashSet<String>,
    },
}

impl VfsListDirectories {
    fn next_working(iter: &mut ReadDir) -> Option<<Self as Iterator>::Item> {
        let mut name = None;
        while name.is_none() {
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

    fn next_zip(
        index: &mut usize,
        prefix: &Box<str>,
        zip: &Arc<Mutex<ZipArchive<VfsBuffer>>>,
        seen: &mut HashSet<String>,
    ) -> Option<<Self as Iterator>::Item> {
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

impl Iterator for VfsListDirectories {
    type Item = Result<String, VfsError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Working(iter) => return Self::next_working(iter),

            Self::ZipArchive {
                index,
                prefix,
                zip,
                seen,
            } => Self::next_zip(index, prefix, zip, seen),
        }
    }
}
