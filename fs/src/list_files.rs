use std::{
    fs::ReadDir,
    sync::{Arc, Mutex},
};

use zip::ZipArchive;

use crate::{buffer::VfsBuffer, VfsError};

pub enum VfsListFiles {
    Working(ReadDir),
    ZipArchive {
        index: usize,
        prefix: Box<str>,
        zip: Arc<Mutex<ZipArchive<VfsBuffer>>>,
    },
}

impl VfsListFiles {
    fn next_working(iter: &mut ReadDir) -> Option<<Self as Iterator>::Item> {
        let mut name = None;
        while name.is_none() {
            match iter.next()?.map_err(Into::into) {
                Ok(dir) => match dir.file_type().map_err(Into::into) {
                    Ok(it) => {
                        if it.is_file() {
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

                if !remaining.contains('/') && file.is_file() {
                    return Some(Ok(remaining.to_string()));
                }
            }
        }

        return None;
    }
}

impl Iterator for VfsListFiles {
    type Item = Result<String, VfsError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Working(iter) => Self::next_working(iter),

            Self::ZipArchive { index, prefix, zip } => Self::next_zip(index, prefix, zip),
        }
    }
}
