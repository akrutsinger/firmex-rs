use binrw::BinRead;

#[derive(Debug, BinRead, Clone)]
#[br(little, magic = b"MRVL")]
pub struct MarvellFirmwareHeader {
    pub unk_const: u32,     // Hardcoded in the official SDK. Constant of 0x2e9cf17b
    pub creation_time: u32, // UNIX timestamp (local timezone)
    #[br(assert(num_segments <= 9))]
    pub num_segments: u32, // Number of program segments. Must be <= 9
    pub elf_version: u32,   // Identical to Elf32.EHdr.e_version
}

impl std::fmt::Display for MarvellFirmwareHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Firmware Header:\n  header_size: {}", self.num_segments)
    }
}

#[derive(Debug, BinRead, Copy, Clone)]
pub struct MarvellSegmentHeader {
    pub segment_type: u32, // Always 2
    pub offset: u32,       // Location of the segment data in this file
    #[br(assert(size % 4 == 0))]
    pub size: u32, // Size of sement data. Must be divisible by 4.
    pub virtual_address: u32, // Virtual memory address
    pub crc_checksum: u32, // CRC-32 checksum of th epadded data segment.
                           // This variant of CRC-32:
                           //  * no preset to 0xffffffff (-1)
                           //  * no post-invert
}

impl std::fmt::Display for MarvellSegmentHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Segment Header:\n  offset: {}\n  size: {}\n  virtual_address: {}\n  crc_checksum: {}",
            self.offset, self.size, self.virtual_address, self.crc_checksum
        )
    }
}
