use std::io::prelude::*;

use std::{fmt, path};
use std::fs;
use std::io;
use std::path::Path;

use zip;
use zip::write::FileOptions;

#[derive(Debug)]
pub struct ArtifactError {
    why: String,
}

impl ArtifactError {
    pub fn new(why: String) -> Self {
        ArtifactError { why }
    }
}

impl std::error::Error for ArtifactError {}

impl fmt::Display for ArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

pub fn zip_files(files_in: Vec<impl AsRef<Path>>,
                 file_out: impl AsRef<Path>,
                 prefix_path: Option<&str>,
                 ro: bool
) {
    let file = match fs::File::create(&file_out) {
        Ok(file) => file,
        Err(why) => panic!("could not create zip archive: {}", why.to_string()),
    };

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(if ro == true { 0o444 } else { 0o777 }); // read-only or full access

    for path in files_in {
        let mut file_bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(why) => panic!("could not create zip archive: {}", why.to_string()),
        };

        // unwrap: path always has a file name at this point
        let path_str = path.as_ref().file_name().unwrap().to_str().unwrap();

        let mut zip_path = String::from(path_str);
        if prefix_path.is_some() {
            zip_path = format!("{}{}", prefix_path.unwrap(), path_str);
        }

        if let Err(why) = zip.start_file_from_path(path::Path::new(&zip_path), options) {
            panic!("could not create zip archive: {}", why.to_string())
        }

        if let Err(why) = zip.write_all(file_bytes.as_mut_slice()) {
            panic!("could not create zip archive: {}", why.to_string())
        }
    }

    println!("🗜 > Wrote zip artifact {}", file_out.as_ref().display());
}

pub fn unzip_to(bytes_in: Vec<u8>, out_dir: &str) -> Result<(), ArtifactError> {
    let mut reader = std::io::Cursor::new(bytes_in);
    let mut archive = zip::ZipArchive::new(reader).unwrap();
    let mut file_out = archive.by_name("terraform").unwrap();

    let mut outfile = fs::File::create(out_dir).unwrap();
    match io::copy(&mut file_out, &mut outfile) {
        Ok(_) => Ok(()),
        Err(why) => Err(ArtifactError::new(why.to_string())),
    }
}
