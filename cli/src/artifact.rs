use std::io::prelude::*;

use std::fs;
use std::path::Path;

use zip;
use zip::write::FileOptions;

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
