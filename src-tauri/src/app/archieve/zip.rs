use std::fs::File;
use std::io;
use std::path::Path;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::app::archieve::ArchiveHandler;

pub struct ZipHandler;

impl ArchiveHandler for ZipHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        let file = File::open(input)?;
        let mut archive = ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = Path::new(output).join(file.name());
            if file.is_dir() {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        Ok(())
    }
    fn compress(&self, input: &str, output: &str) -> io::Result<()> {
        let path = Path::new(input);
        let file = File::create(output)?;
        let mut zip = ZipWriter::new(file);
        let options =
            FileOptions::<'_, ()>::default().compression_method(zip::CompressionMethod::Deflated);
        if path.is_file() {
            let name = path.file_name().unwrap().to_string_lossy();
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        } else if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry?;
                let p = entry.path();
                let name = p.strip_prefix(path).unwrap().to_string_lossy();
                if p.is_file() {
                    zip.start_file(name.clone(), options)?;
                    let mut f = File::open(p)?;
                    std::io::copy(&mut f, &mut zip)?;
                } else if !name.is_empty() {
                    zip.add_directory(name, options)?;
                }
            }
        }
        zip.finish()?;
        Ok(())
    }
}
