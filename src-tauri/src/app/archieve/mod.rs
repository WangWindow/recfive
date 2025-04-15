mod external;
mod gz;
mod rar;
mod sevenz;
mod tar;
mod tgz;
mod txz;
mod zip;

use std::io;
use std::path::Path;

pub trait ArchiveHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()>;
    fn compress(&self, input: &str, output: &str) -> io::Result<()>;
}

pub fn get_handler(ext: &str) -> Option<Box<dyn ArchiveHandler>> {
    match ext.to_ascii_lowercase().as_str() {
        "zip" => Some(Box::new(zip::ZipHandler)),
        "tar" => Some(Box::new(tar::TarHandler)),
        "gz" => Some(Box::new(gz::GzHandler)),
        "tar.gz" | "tgz" => Some(Box::new(tgz::TgzHandler)),
        "tar.xz" | "txz" => Some(Box::new(txz::TxzHandler)),
        "7z" => Some(Box::new(sevenz::SevenZHandler)),
        "rar" => Some(Box::new(rar::RarHandler)),
        _ => None,
    }
}

pub fn extract_archive(input: &str, output: &str) -> io::Result<()> {
    let ext = Path::new(input)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if let Some(handler) = get_handler(ext) {
        handler.extract(input, output)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "不支持的压缩格式"))
    }
}

pub fn compress_archive(input: &str, output: &str) -> io::Result<()> {
    let ext = Path::new(output)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if let Some(handler) = get_handler(ext) {
        handler.compress(input, output)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "不支持的压缩格式"))
    }
}
