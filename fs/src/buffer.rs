use std::{
    fmt::Debug,
    fs::File,
    io::{Cursor, Read, Seek},
};

#[derive(Debug)]
pub enum VfsBuffer {
    Bytes(Cursor<Box<[u8]>>),
    File(File),
}

impl Read for VfsBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        return match self {
            Self::Bytes(cursor) => cursor.read(buf),
            Self::File(file) => file.read(buf),
        };
    }
}

impl Seek for VfsBuffer {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        return match self {
            Self::Bytes(cursor) => cursor.seek(pos),
            Self::File(file) => file.seek(pos),
        };
    }
}

impl<const N: usize> From<[u8; N]> for VfsBuffer {
    fn from(value: [u8; N]) -> Self {
        return Self::from(value.to_vec());
    }
}

impl From<&[u8]> for VfsBuffer {
    fn from(value: &[u8]) -> Self {
        return Self::from(value.to_vec());
    }
}

impl From<Vec<u8>> for VfsBuffer {
    fn from(value: Vec<u8>) -> Self {
        return Self::from(value.into_boxed_slice());
    }
}

impl From<Box<[u8]>> for VfsBuffer {
    fn from(value: Box<[u8]>) -> Self {
        return Self::Bytes(Cursor::new(value));
    }
}

impl From<File> for VfsBuffer {
    fn from(value: File) -> Self {
        return Self::File(value);
    }
}
