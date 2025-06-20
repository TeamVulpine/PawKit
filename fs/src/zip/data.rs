use crate::zip::{read_from, ZipArchiveError};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EndOfCentralDirectory {
    pub signature: u32,
    pub current_disk: u16,
    pub directory_disk: u16,
    pub this_entries: u16,
    pub total_entries: u16,
    pub directory_size: u32,
    pub directory_offset: u32,
    pub comment_size: u16,
}

impl EndOfCentralDirectory {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub const SIGNATURE_BYTES: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];

    pub fn is_zip64(&self) -> bool {
        return self.current_disk == 0xffff
            && self.directory_disk == 0xffff
            && self.this_entries == 0xffff
            && self.total_entries == 0xffff
            && self.directory_size == 0xffff
            && self.directory_offset == 0xffff;
    }

    pub const SIGNATURE: u32 = 0x06054b50;

    pub fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EndOfCentralDirectory64Locator {
    pub signature: u32,
    pub disk: u32,
    pub eocd64_offset: u64,
    pub disks: u32,
}

impl EndOfCentralDirectory64Locator {
    pub const SIZE: usize = size_of::<Self>();

    pub const SIGNATURE: u32 = 0x07064b50;

    pub fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EndOfCentralDirectory64 {
    pub signature: u32,
    pub size: u64,
    pub version: u16,
    pub min_version: u64,
    pub current_disk: u32,
    pub directory_disk: u32,
    pub this_entries: u32,
    pub total_entries: u32,
    pub directory_size: u64,
    pub directory_offset: u64,
}

impl EndOfCentralDirectory64 {
    pub const SIGNATURE: u32 = 0x06064b50;

    pub fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CentralDirectoryHeader {
    pub signature: u32,
    pub curr_version: u16,
    pub min_version: u16,
    pub flags: u16,
    pub compression_method: u16,
    pub last_modified_time: u16,
    pub last_modified_date: u16,
    pub checksum: u32,
    pub size_compressed: u32,
    pub size_decompressed: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub comment_length: u16,
    pub disk: u16,
    pub internal_attributes: u16,
    pub external_attributes: u32,
    pub offset: u32,
}

impl CentralDirectoryHeader {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const SIGNATURE: u32 = 0x02014b50;

    pub fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LocalFileHeader {
    pub signature: u32,
    pub min_version: u16,
    pub flags: u16,
    pub compression_method: u16,
    pub last_modified_time: u16,
    pub last_modified_date: u16,
    pub checksum: u32,
    pub size_compressed: u32,
    pub size_decompressed: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
}

impl LocalFileHeader {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const SIGNATURE: u32 = 0x04034b50;

    pub fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ExtraFieldHeader {
    pub kind: u16,
    pub size: u16,
}

impl ExtraFieldHeader {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

pub struct Zip64ExtraField {
    pub size_decompressed: u64,
    pub size_compressed: u64,
    pub offset: u64,
}

impl Zip64ExtraField {
    pub fn defaulted(record: &CentralDirectoryHeader) -> Self {
        return Self {
            size_decompressed: record.size_decompressed as u64,
            size_compressed: record.size_compressed as u64,
            offset: record.offset as u64,
        };
    }

    pub fn parse(record: &CentralDirectoryHeader, data: &[u8], offset: usize) -> Self {
        if (record.extra_field_length as usize) < ExtraFieldHeader::SIZE {
            return Self::defaulted(record);
        }

        let mut extra_offset =
            offset + CentralDirectoryHeader::SIZE + record.file_name_length as usize;
        let end = offset
            + CentralDirectoryHeader::SIZE
            + record.file_name_length as usize
            + record.extra_field_length as usize;

        while extra_offset + ExtraFieldHeader::SIZE <= end {
            let extra_field_header = read_from::<ExtraFieldHeader>(data, extra_offset);

            if extra_field_header.kind != 1 {
                extra_offset += ExtraFieldHeader::SIZE + extra_field_header.size as usize;
                continue;
            }

            let extra_field_data_start = extra_offset + ExtraFieldHeader::SIZE;

            let (size_decompressed, new_offset) = if record.size_decompressed == 0xFFFFFFFF {
                (
                    *read_from::<u64>(data, extra_field_data_start),
                    extra_field_data_start + 8,
                )
            } else {
                (record.size_decompressed as u64, extra_field_data_start)
            };

            let (size_compressed, new_offset) = if record.size_compressed == 0xFFFFFFFF {
                (*read_from::<u64>(data, new_offset), new_offset + 8)
            } else {
                (record.size_compressed as u64, new_offset)
            };

            let data_offset = if record.offset == 0xFFFFFFFF {
                *read_from::<u64>(data, new_offset)
            } else {
                record.offset as u64
            };

            return Self {
                size_decompressed,
                size_compressed,
                offset: data_offset,
            };
        }

        Self::defaulted(record)
    }
}
