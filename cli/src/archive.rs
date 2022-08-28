use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use zip;
use zip::write::FileOptions;

#[derive(Debug)]
pub struct ArchiveError {
    why: String,
}

impl ArchiveError {
    pub fn new(why: String) -> Self {
        ArchiveError { why }
    }
}

impl std::error::Error for ArchiveError {}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

pub fn zip_dirs(
    dirs_in: Vec<PathBuf>,
    file_out: impl AsRef<Path>,
    exclude_files_named: Vec<&str>,
) -> Result<(), ()> {
    use walkdir::WalkDir;

    let file = match fs::File::create(&file_out) {
        Ok(file) => file,
        Err(why) => panic!("could not create zip archive: {}", why.to_string()),
    };

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for dir in dirs_in {
        for entry in WalkDir::new(dir.clone()).into_iter().filter_map(|e| e.ok()) {
            if exclude_files_named
                .iter()
                .find(|&f| f == &entry.file_name().to_str().unwrap())
                .is_none()
            {
                let mut root_dir_up = dir.clone();
                root_dir_up.pop();
                let zip_path = entry
                    .path()
                    .to_str()
                    .unwrap()
                    .replace(root_dir_up.to_str().unwrap(), "");

                let mut file_bytes = match fs::read(&entry.path()) {
                    Ok(bytes) => bytes,
                    Err(_) => continue,
                };

                zip.start_file(&format!("{}", zip_path), options)
                    .expect("could not create zip archive");
                zip.write_all(file_bytes.as_mut_slice())
                    .expect("could not create zip archive");
            }
        }
    }

    println!("🗜  > Wrote zip artifact {}", file_out.as_ref().display());

    Ok(())
}

pub fn unzip_terraform(bytes_in: Vec<u8>, out_dir: &str) -> Result<(), ArchiveError> {
    let reader = std::io::Cursor::new(bytes_in);
    let mut archive = zip::ZipArchive::new(reader).unwrap();
    let mut file_out = archive.by_name("terraform").unwrap();

    let mut outfile = fs::File::create(out_dir).unwrap();
    match io::copy(&mut file_out, &mut outfile) {
        Ok(_) => Ok(()),
        Err(why) => Err(ArchiveError::new(why.to_string())),
    }
}

pub fn unzip(bytes_in: &[u8], out_dir: &str) -> Result<(), ArchiveError> {
    println!("Unzipping archive in {}...", out_dir);
    let reader = std::io::Cursor::new(bytes_in);
    let mut archive = zip::ZipArchive::new(reader).unwrap();
    archive.extract(out_dir).unwrap();
    println!("...done!");
    Ok(())
}
