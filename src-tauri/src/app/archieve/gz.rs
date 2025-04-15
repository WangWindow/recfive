use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io::{self, copy, BufReader, BufWriter, Read};

use crate::app::archieve::ArchiveHandler;

pub struct GzHandler;

impl ArchiveHandler for GzHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        let input = File::open(input)?;
        let buffered = BufReader::new(input);
        let mut decoder = GzDecoder::new(buffered);
        let output = File::create(output)?;
        let mut writer = BufWriter::new(output);
        copy(&mut decoder, &mut writer)?;
        Ok(())
    }
    fn compress(&self, input: &str, output: &str) -> io::Result<()> {
        let input = File::open(input)?;
        let buffered = BufReader::new(input);
        let output = File::create(output)?;
        let mut encoder = GzEncoder::new(output, Compression::default());
        copy(&mut buffered.take(u64::MAX), &mut encoder)?;
        Ok(())
    }
}
