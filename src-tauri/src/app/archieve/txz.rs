use lzma_rs::xz_decompress;
use std::fs::File;
use std::io::{self, BufReader};
use tar::Archive as TarArchive;

use crate::app::archieve::ArchiveHandler;

pub struct TxzHandler;

impl ArchiveHandler for TxzHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        let file = File::open(input)?;
        let mut xz_decoded: Vec<u8> = Vec::new();
        xz_decompress(&mut BufReader::new(file), &mut xz_decoded)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
        let mut archive = TarArchive::new(&xz_decoded[..]);
        archive.unpack(output)?;
        Ok(())
    }
    fn compress(&self, _input: &str, _output: &str) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "txz压缩暂不支持"))
    }
}
