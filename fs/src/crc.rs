const CRC32_POLYNOMIAL: u32 = 0xEDB88320;

const fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i: usize = 0;
    while i < 256 {
        let mut crc = i as u32;

        let mut j: usize = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC32_POLYNOMIAL;
            } else {
                crc >>= 1;
            }

            j += 1;
        }
        table[i] = crc;

        i += 1;
    }
    return table;
}

const CRC32_TABLE: [u32; 256] = generate_crc32_table();

pub fn verify_checksum(data: &[u8], checksum: u32) -> bool {
    let mut crc = 0xffffffff;

    for &byte in data {
        let index = ((crc ^ (byte as u32)) & 0xFF) as usize;
        crc = CRC32_TABLE[index] ^ (crc >> 8);
    }

    let calculated = !crc;

    return calculated == checksum;
}
