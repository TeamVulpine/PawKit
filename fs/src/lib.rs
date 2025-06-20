use crate::zip::ZipArchive;

pub mod crc;
pub mod zip;

pub enum Filesystem {
    Working,
    Zip(ZipArchive),
}
