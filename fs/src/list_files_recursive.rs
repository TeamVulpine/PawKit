use crate::{Vfs, VfsError, VfsListDirectories, VfsListFiles};

pub struct VfsListFilesRecursive {
    pub(crate) stack: Vec<(Vfs, VfsListDirectories)>,
    pub(crate) files: Option<VfsListFiles>,
    pub(crate) prefix: Box<str>,
}

impl VfsListFilesRecursive {
    fn next_file(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(files) = &mut self.files {
            if let Some(file) = files.next() {
                match file {
                    Ok(file) => {
                        let prefix = self
                            .stack
                            .last()
                            .map(|it| it.0.subdirectory.as_ref())
                            .unwrap_or(None)
                            .unwrap_or(&self.prefix);

                        let prefix = &prefix[self.prefix.len()..];

                        let prefix = prefix.strip_prefix("/").unwrap_or(prefix);

                        return Some(Ok(if prefix.is_empty() {
                            file
                        } else {
                            format!("{}/{}", prefix, file)
                        }));
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
        }

        return None;
    }

    fn next_dir(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some((vfs, dirs)) = self.stack.last_mut() {
            match dirs.next() {
                Some(Ok(dir)) => {
                    let new_vfs = match vfs.subdirectory(&dir) {
                        Ok(value) => value,
                        Err(err) => return Some(Err(err)),
                    };

                    let new_dirs = match new_vfs.list_subdirectories() {
                        Ok(value) => value,
                        Err(err) => return Some(Err(err)),
                    };

                    let new_files = match new_vfs.list_files() {
                        Ok(value) => value,
                        Err(err) => return Some(Err(err)),
                    };

                    self.stack.push((new_vfs, new_dirs));
                    self.files = Some(new_files);
                    break;
                }

                Some(Err(err)) => return Some(Err(err)),

                None => {
                    self.stack.pop();
                }
            }
        }

        return None;
    }
}

impl Iterator for VfsListFilesRecursive {
    type Item = Result<String, VfsError>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            if let Some(value) = self.next_file() {
                return Some(value);
            }

            if let Some(value) = self.next_dir() {
                return Some(value);
            }
        }

        return None;
    }
}
