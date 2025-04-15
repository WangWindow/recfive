use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io;
use tar::{Archive as TarArchive, Builder as TarBuilder};

use crate::app::archieve::ArchiveHandler;

pub struct TgzHandler;

impl ArchiveHandler for TgzHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        let file = File::open(input)?;
        let gz = GzDecoder::new(file);
        let mut archive = TarArchive::new(gz);
        archive.unpack(output)?;
        Ok(())
    }
    fn compress(&self, input: &str, output: &str) -> io::Result<()> {
        let file = File::create(output)?;
        let gz = GzEncoder::new(file, Compression::default());
        let mut builder = TarBuilder::new(gz);
        builder.append_dir_all(".", input)?;
        Ok(())
    }
}
