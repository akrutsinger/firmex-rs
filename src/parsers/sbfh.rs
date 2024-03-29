use binrw::BinRead;

#[derive(Debug, BinRead, Clone)]
#[br(little, magic = b"SBFH")]
pub struct FileHeader {
    pub header_size: u32,
    #[br(count = 7)]
    pub unknown1: Vec<u8>,
    pub firmware_size: u32,
    #[br(count = header_size - 4 - 4 - 7 - 4)]
    pub unknown2: Vec<u8>, // The entire header size includes the u32 of the size and the 4-byte signature.
}

impl std::fmt::Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File Header:\n  header_size: {:#x}\n  firmware_size: {:#x}",
            self.header_size, self.firmware_size
        )
    }
}
