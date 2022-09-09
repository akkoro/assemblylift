pub mod project;

use std::io::Write;
use std::path::PathBuf;
use std::{fs, io, path};

use handlebars::Handlebars;
use serde_json::value::{Map as SerdeMap, Value as Json};

pub struct Document {
    pub file_name: &'static str,
    pub document: String,
}

pub fn write_documents(path: &PathBuf, docs: &Vec<Document>, data: &mut SerdeMap<String, Json>) {
    let mut reg = Handlebars::new();
    for doc in docs {
        reg.register_template_string(doc.file_name, doc.document.clone())
            .unwrap();

        let render = reg.render(doc.file_name, &data).unwrap();
        let mut path = PathBuf::from(path);
        path.push(doc.file_name);
        write_to_file(&*path, render).unwrap();
    }
}

fn write_to_file(path: &path::Path, contents: String) -> Result<(), io::Error> {
    fs::create_dir_all(path.parent().unwrap())?;

    let mut file = match fs::File::create(path) {
        Err(why) => panic!(
            "couldn't create file {}: {}",
            path.display(),
            why.to_string()
        ),
        Ok(file) => file,
    };

    println!("📄 > Wrote {}", path.display());
    file.write_all(contents.as_bytes())
}
