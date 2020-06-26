use std::io::prelude::*;

use std::fmt;
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

pub fn zip_files(files_in: Vec<&Path>, file_out: &Path) {
    let file = match fs::File::create(file_out) {
        Ok(file) => file,
        Err(why) => panic!("could not create zip archive: {}", why.to_string()),
    };

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o777); // full access

    for path in files_in {
        let mut file_bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(why) => panic!("could not create zip archive: {}", why.to_string()),
        };

        // unwrap: path always has a file name at this point
        let path_str = path.file_name().unwrap().to_str().unwrap();

        if let Err(why) = zip.start_file(path_str, options) {
            panic!("could not create zip archive: {}", why.to_string())
        }

        if let Err(why) = zip.write_all(file_bytes.as_mut_slice()) {
            panic!("could not create zip archive: {}", why.to_string())
        }
    }

    println!("🗜 > Wrote zip artifact {}", file_out.display());
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
