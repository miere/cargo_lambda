use crate::error::{Result};

use std::io::{Seek, Write};
use std::fs;
use std::fs::File;
use zip::write::{FileOptions, ZipWriter};

pub fn package_binary(zip_file_path: &str, bootstrap_file_path: &str) -> Result<()> {
    let mut file = File::create(zip_file_path)?;
    let content = fs::read(bootstrap_file_path)?;
    create_zip_package(&mut file, &content)
}

fn create_zip_package<T: Seek + Write>(buf: &mut T, content: &[u8]) -> Result<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file("bootstrap", FileOptions::default())?;
    writer.write(content)?;
    writer.finish()?;
    Ok(())
}
