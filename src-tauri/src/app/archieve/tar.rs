use std::fs::File;
use std::io;
use tar::{Archive as TarArchive, Builder as TarBuilder};

use crate::app::archieve::ArchiveHandler;

pub struct TarHandler;

impl ArchiveHandler for TarHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        let file = File::open(input)?;
        let mut archive = TarArchive::new(file);
        archive.unpack(output)?;
        Ok(())
    }
    fn compress(&self, input: &str, output: &str) -> io::Result<()> {
        let file = File::create(output)?;
        let mut builder = TarBuilder::new(file);
        builder.append_dir_all(".", input)?;
        Ok(())
    }
}
