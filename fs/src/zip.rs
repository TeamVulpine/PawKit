use crate::crc;

fn read_from<T>(bytes: &[u8], offset: usize) -> &T
where
    T: bytemuck::Pod + bytemuck::Zeroable + Sized,
{
    bytemuck::from_bytes::<T>(&bytes[offset..offset + size_of::<T>()])
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct EndOfCentralDirectory {
    signature: u32,
    current_disk: u16,
    directory_disk: u16,
    this_entries: u16,
    total_entries: u16,
    directory_size: u32,
    directory_offset: u32,
    comment_size: u16,
}

impl EndOfCentralDirectory {
    const SIZE: usize = std::mem::size_of::<Self>();

    const SIGNATURE_BYTES: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];

    fn is_zip64(&self) -> bool {
        return self.current_disk == 0xffff
            && self.directory_disk == 0xffff
            && self.this_entries == 0xffff
            && self.total_entries == 0xffff
            && self.directory_size == 0xffff
            && self.directory_offset == 0xffff;
    }

    const SIGNATURE: u32 = 0x06054b50;

    fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct EndOfCentralDirectory64Locator {
    signature: u32,
    disk: u32,
    eocd64_offset: u64,
    disks: u32,
}

impl EndOfCentralDirectory64Locator {
    const SIZE: usize = size_of::<Self>();

    const SIGNATURE: u32 = 0x07064b50;

    fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct EndOfCentralDirectory64 {
    signature: u32,
    size: u64,
    version: u16,
    min_version: u64,
    current_disk: u32,
    directory_disk: u32,
    this_entries: u32,
    total_entries: u32,
    directory_size: u64,
    directory_offset: u64,
}

impl EndOfCentralDirectory64 {
    const SIGNATURE: u32 = 0x06064b50;

    fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct CentralDirectoryHeader {
    signature: u32,
    curr_version: u16,
    min_version: u16,
    flags: u16,
    compression_method: u16,
    last_modified_time: u16,
    last_modified_date: u16,
    checksum: u32,
    size_compressed: u32,
    size_decompressed: u32,
    file_name_length: u16,
    extra_field_length: u16,
    comment_length: u16,
    disk: u16,
    internal_attributes: u16,
    external_attributes: u32,
    offset: u32,
}

impl CentralDirectoryHeader {
    const SIZE: usize = std::mem::size_of::<Self>();
    const SIGNATURE: u32 = 0x02014b50;

    fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct LocalFileHeader {
    signature: u32,
    min_version: u16,
    flags: u16,
    compression_method: u16,
    last_modified_time: u16,
    last_modified_date: u16,
    checksum: u32,
    size_compressed: u32,
    size_decompressed: u32,
    file_name_length: u16,
    extra_field_length: u16,
}

impl LocalFileHeader {
    const SIZE: usize = std::mem::size_of::<Self>();
    const SIGNATURE: u32 = 0x04034b50;

    fn verify(&self) -> Result<(), ZipArchiveError> {
        if self.signature != Self::SIGNATURE {
            return Err(ZipArchiveError::InvalidSignature);
        }
        return Ok(());
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct ExtraFieldHeader {
    kind: u16,
    size: u16,
}

impl ExtraFieldHeader {
    const SIZE: usize = std::mem::size_of::<Self>();
}

struct Zip64ExtraField {
    size_decompressed: u64,
    size_compressed: u64,
    offset: u64,
}

impl Zip64ExtraField {
    fn defaulted(record: &CentralDirectoryHeader) -> Self {
        return Self {
            size_decompressed: record.size_decompressed as u64,
            size_compressed: record.size_compressed as u64,
            offset: record.offset as u64,
        };
    }

    fn parse(record: &CentralDirectoryHeader, data: &[u8], offset: usize) -> Self {
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

#[derive(Debug)]
enum CompressionMethod {
    Uncompressed,
    Shrunk,
    Reduced1,
    Reduced2,
    Reduced3,
    Imploded,
    Bzip2,
    Lzma,
    WavPack,
    Ppmd,
    Xz,
}

#[derive(Debug)]
pub enum CompressionError {
    UnimplementedCompressionMethod,
    ChecksumMismatch,
}

impl CompressionMethod {
    const RAW_UNCOMPRESSED: u16 = 0;
    const RAW_SHRUNK: u16 = 1;
    const RAW_REDUCED_1: u16 = 2;
    const RAW_REDUCED_2: u16 = 3;
    const RAW_REDUCED_3: u16 = 4;
    const RAW_IMPLODED: u16 = 5;
    const RAW_BZIP_2: u16 = 10;
    const RAW_LZMA: u16 = 11;
    const RAW_WAV_PACK: u16 = 14;
    const RAW_PPMD: u16 = 15;
    const RAW_XZ: u16 = 19;

    fn from(value: u16) -> Option<Self> {
        return Some(match value {
            Self::RAW_UNCOMPRESSED => Self::Uncompressed,
            Self::RAW_SHRUNK => Self::Shrunk,
            Self::RAW_REDUCED_1 => Self::Reduced1,
            Self::RAW_REDUCED_2 => Self::Reduced2,
            Self::RAW_REDUCED_3 => Self::Reduced3,
            Self::RAW_IMPLODED => Self::Imploded,
            Self::RAW_BZIP_2 => Self::Bzip2,
            Self::RAW_LZMA => Self::Lzma,
            Self::RAW_WAV_PACK => Self::WavPack,
            Self::RAW_PPMD => Self::Ppmd,
            Self::RAW_XZ => Self::Xz,

            _ => return None,
        });
    }

    fn verify_checksum(data: Vec<u8>, checksum: u32) -> Result<Vec<u8>, CompressionError> {
        if !crc::verify_checksum(&data, checksum) {
            return Err(CompressionError::ChecksumMismatch);
        }

        return Ok(data);
    }

    fn decompress(&self, data: &[u8], checksum: u32) -> Result<Vec<u8>, CompressionError> {
        return match self {
            Self::Uncompressed => Self::verify_checksum(data.to_vec(), checksum),

            _ => Err(CompressionError::UnimplementedCompressionMethod),
        };
    }
}

#[derive(Debug)]
struct FileEntry {
    name: String,
    decompressed_size: u64,
    checksum: u32,
    compression_method: CompressionMethod,
    data: Box<[u8]>,
}

#[derive(Debug)]
pub struct ZipArchive {
    entries: Vec<FileEntry>,
}

#[derive(Debug)]
pub enum ZipArchiveError {
    EndOfCentralDirectoryNotFound,
    CentralDirectoryOutOfBounds,
    CentralDirectoryHeaderOutOfBounds,
    InvalidUtf8,
    LocalFileHeaderOutOfBounds,
    FileDataOutOfBounds,
    InvalidCompressionMethod,
    FileNotFound,
    FilesizeMismatch,
    InvalidSignature,
    CompressionError(CompressionError),
}

impl From<CompressionError> for ZipArchiveError {
    fn from(value: CompressionError) -> Self {
        return Self::CompressionError(value);
    }
}

impl ZipArchive {
    fn parse_central(
        file_bytes: &[u8],
        offset: usize,
    ) -> Result<(FileEntry, usize), ZipArchiveError> {
        let file_len = file_bytes.len();

        if offset + CentralDirectoryHeader::SIZE > file_len {
            return Err(ZipArchiveError::CentralDirectoryHeaderOutOfBounds);
        }

        let central_dir_header = read_from::<CentralDirectoryHeader>(file_bytes, offset);
        central_dir_header.verify()?;

        let ext_data = Zip64ExtraField::parse(central_dir_header, file_bytes, offset);

        let offset = offset + CentralDirectoryHeader::SIZE;

        let file_name_end = offset + central_dir_header.file_name_length as usize;
        let file_name_bytes = &file_bytes[offset..file_name_end];
        let file_name = str::from_utf8(file_name_bytes)
            .map_err(|_| ZipArchiveError::InvalidUtf8)?
            .to_string();
        let offset = file_name_end;

        let offset = offset + central_dir_header.extra_field_length as usize;
        let offset = offset + central_dir_header.comment_length as usize;

        let local_header_offset = ext_data.offset as usize;
        if local_header_offset + LocalFileHeader::SIZE > file_len {
            return Err(ZipArchiveError::LocalFileHeaderOutOfBounds);
        }

        let local_header = read_from::<LocalFileHeader>(file_bytes, local_header_offset);
        local_header.verify()?;
        let file_data_start = local_header_offset
            + LocalFileHeader::SIZE
            + local_header.file_name_length as usize
            + local_header.extra_field_length as usize;

        let file_data_end = file_data_start + ext_data.size_compressed as usize;
        if file_data_end > file_len {
            return Err(ZipArchiveError::FileDataOutOfBounds);
        }

        let compressed_data = &file_bytes[file_data_start..file_data_end];

        let compression_method = CompressionMethod::from(central_dir_header.compression_method)
            .ok_or(ZipArchiveError::InvalidCompressionMethod)?;

        return Ok((
            FileEntry {
                name: file_name,
                decompressed_size: ext_data.size_decompressed,
                checksum: central_dir_header.checksum,
                compression_method,
                data: compressed_data.to_vec().into_boxed_slice(),
            },
            offset,
        ));
    }

    fn from_pos(
        file_bytes: &[u8],
        mut offset: usize,
        directory_size: usize,
        this_entries: usize,
    ) -> Result<Self, ZipArchiveError> {
        let mut entries = vec![];

        if offset + directory_size as usize > file_bytes.len() {
            return Err(ZipArchiveError::CentralDirectoryOutOfBounds);
        }

        for _ in 0..this_entries {
            let (file_entry, new_offset) = Self::parse_central(file_bytes, offset)?;

            offset = new_offset;

            entries.push(file_entry);
        }

        return Ok(Self { entries });
    }

    fn from_zip64(
        file_bytes: &[u8],
        eocd64_locator: &EndOfCentralDirectory64Locator,
    ) -> Result<Self, ZipArchiveError> {
        let eocd =
            read_from::<EndOfCentralDirectory64>(file_bytes, eocd64_locator.eocd64_offset as usize);
        eocd.verify()?;

        return Self::from_pos(
            file_bytes,
            eocd.directory_offset as usize,
            eocd.directory_size as usize,
            eocd.this_entries as usize,
        );
    }

    fn from_zip32(
        file_bytes: &[u8],
        eocd: &EndOfCentralDirectory,
    ) -> Result<Self, ZipArchiveError> {
        return Self::from_pos(
            file_bytes,
            eocd.directory_offset as usize,
            eocd.directory_size as usize,
            eocd.this_entries as usize,
        );
    }

    pub fn from(file_bytes: &[u8]) -> Result<Self, ZipArchiveError> {
        let is_zip64 = file_bytes.len() > 0xFFFFFFFF;

        let file_len = file_bytes.len();

        let eocd_signature = EndOfCentralDirectory::SIGNATURE_BYTES;
        let mut eocd_offset = None;

        if file_len < EndOfCentralDirectory::SIZE {
            return Err(ZipArchiveError::EndOfCentralDirectoryNotFound);
        }

        for i in (0..=(file_len - EndOfCentralDirectory::SIZE)).rev() {
            if &file_bytes[i..i + 4] == eocd_signature {
                eocd_offset = Some(i);
                break;
            }
        }

        let eocd_offset = eocd_offset.ok_or(ZipArchiveError::EndOfCentralDirectoryNotFound)?;

        let eocd = read_from::<EndOfCentralDirectory>(file_bytes, eocd_offset);
        eocd.verify()?;

        let is_zip64 = is_zip64 || eocd.is_zip64();

        if !is_zip64 {
            return Self::from_zip32(file_bytes, eocd);
        }

        let locator_offset = eocd_offset - EndOfCentralDirectory64Locator::SIZE;

        let eocd_locator = read_from::<EndOfCentralDirectory64Locator>(file_bytes, locator_offset);
        eocd_locator.verify()?;

        return Self::from_zip64(file_bytes, eocd_locator);
    }

    fn get_entry(&self, name: &str) -> Option<&FileEntry> {
        for entry in &self.entries {
            if entry.name.eq_ignore_ascii_case(name) {
                return Some(entry);
            }
        }
        return None;
    }

    pub fn get_file_raw(&self, name: &str) -> Result<Vec<u8>, ZipArchiveError> {
        let Some(entry) = self.get_entry(name) else {
            return Err(ZipArchiveError::FileNotFound);
        };

        let data = entry
            .compression_method
            .decompress(&entry.data, entry.checksum)?;

        if data.len() != entry.decompressed_size as usize {
            return Err(ZipArchiveError::FilesizeMismatch);
        }

        return Ok(data);
    }

    pub fn get_file_raw_str(&self, name: &str) -> Result<String, ZipArchiveError> {
        let data = self.get_file_raw(name)?;

        return str::from_utf8(&data)
            .map_err(|_| ZipArchiveError::InvalidUtf8)
            .map(|it| it.to_string());
    }
}
