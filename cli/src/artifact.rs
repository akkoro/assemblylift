use std::io::prelude::*;

use std::fs;
use std::path::Path;

use zip;
use zip::write::FileOptions;

pub fn zip_files(files_in: Vec<&Path>, file_out: &Path) {
    let error_handler = |why: String| panic!("could not create zip archive: {}", why);

    let file = match fs::File::create(file_out) {
        Ok(file) => file,
        Err(why) => error_handler(why.to_string())
    };

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o777); // full access

    for path in files_in {
        let mut file_bytes = fs::read(path).unwrap();
        let path_str = path.file_name().unwrap().to_str().unwrap();

        if let Err(why) = zip.start_file(path_str, options) {
            error_handler(why.to_string());
        }

        if let Err(why) = zip.write_all(file_bytes.as_mut_slice()) {
            error_handler(why.to_string());
        }
    }

    println!("🗜️ Wrote zip artifact {}", file_out.display());
}
