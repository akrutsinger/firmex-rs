pub(crate) mod mrvl;
pub(crate) mod sbfh;

use self::mrvl::{MarvellFirmwareHeader, MarvellSegmentHeader};
use binrw::BinReaderExt;
use sbfh::FileHeader;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Error while trying to read data: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Error while trying to read data: {0}")]
    BinReadError(#[from] binrw::Error),
    #[error("Invalid signature, provided file is not a known file")]
    InvalidSignature,
}

#[derive(Debug)]
pub struct SBFH {
    pub file_header: FileHeader,
    pub firmware_data: Vec<u8>,
}

impl SBFH {
    pub fn read(path: &str) -> Result<SBFH, Error> {
        let path = Path::new(path);
        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        // Read all of the parts of the SBFH file
        let file_header: FileHeader = reader.read_le()?;

        let mut firmware_data: Vec<u8> = vec![0u8; file_header.firmware_size as usize];

        reader.read_exact(&mut firmware_data)?;

        let sbfh = SBFH {
            file_header,
            firmware_data,
        };

        Ok(sbfh)
    }
}

impl std::fmt::Display for SBFH {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n", self.file_header)
    }
}

#[derive(Debug)]
pub struct MRVL {
    // NOTE: not sure if all of the fields being public is the best approach.
    pub firmware_header: MarvellFirmwareHeader,
    pub firmware_segments: Vec<MarvellSegmentHeader>,
    pub firmware_data: Vec<Vec<u8>>,
}

impl MRVL {
    pub fn read(path: &str) -> Result<MRVL, Error> {
        let path = Path::new(path);
        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        // Read all of the parts of the MRVL file
        let firmware_header: MarvellFirmwareHeader = reader.read_le()?;
        let mut firmware_segments: Vec<MarvellSegmentHeader> =
            Vec::with_capacity(firmware_header.num_segments as usize);
        for _ in 0..firmware_header.num_segments {
            let segment: MarvellSegmentHeader = reader.read_le()?;
            firmware_segments.push(segment);
        }

        let mut firmware_data: Vec<Vec<u8>> =
            Vec::with_capacity(firmware_header.num_segments as usize);
        for segment in &firmware_segments {
            let mut data: Vec<u8> = vec![0u8; segment.size as usize];

            reader.seek(SeekFrom::Start(segment.offset as u64))?;

            reader.read_exact(&mut data)?;

            firmware_data.push(data);
        }

        let mrvl = MRVL {
            firmware_header,
            firmware_segments,
            firmware_data,
        };

        Ok(mrvl)
    }
}

impl std::fmt::Display for MRVL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut segment_output = String::new();
        for s in &self.firmware_segments {
            segment_output.push_str(s.to_string().as_str());
            segment_output.push_str("\n");
        }
        write!(f, "{}\n{}", self.firmware_header, segment_output)
    }
}
