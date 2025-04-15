use crate::app::archieve::ArchiveHandler;
use std::io;

pub struct RarHandler;

impl ArchiveHandler for RarHandler {
    fn extract(&self, input: &str, output: &str) -> io::Result<()> {
        std::process::Command::new("7z.exe")
            .args(["x", input, &format!("-o{}", output), "-y"])
            .status()
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "rar解压失败"))
                }
            })
    }
    fn compress(&self, input: &str, output: &str) -> io::Result<()> {
        std::process::Command::new("7z.exe")
            .args(["a", output, input])
            .status()
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "rar压缩失败"))
                }
            })
    }
}
