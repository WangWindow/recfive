use std::io;
use crate::app::archieve::ArchiveHandler;

pub struct SevenZHandler;

impl ArchiveHandler for SevenZHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        std::process::Command::new("7z.exe")
            .args(["x", input, &format!("-o{}", output), "-y"])
            .status()
            .and_then(|status| if status.success() { Ok(()) } else { Err(io::Error::new(io::ErrorKind::Other, "7z解压失败")) })
    }
    fn compress(&self, input: &str, output: &str) -> io::Result<()> {
        std::process::Command::new("7z.exe")
            .args(["a", output, input])
            .status()
            .and_then(|status| if status.success() { Ok(()) } else { Err(io::Error::new(io::ErrorKind::Other, "7z压缩失败")) })
    }
}