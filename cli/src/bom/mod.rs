use std::io::Write;
use std::path::PathBuf;
use std::{fs, io, path};

use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::value::{Map, Value as Json};

pub mod function;
pub mod manifest;
pub mod service;

pub struct Document {
    pub file_name: &'static str,
    pub document: String,
}

pub trait DocumentSet<'a, M: Deserialize<'a>> {
    fn file_names() -> Vec<Document>;
    fn read(path: &PathBuf) -> M;
    fn write(path: &PathBuf, data: &mut Map<String, Json>);
}

pub(in crate::bom) fn write_documents(
    path: &PathBuf,
    docs: Vec<Document>,
    data: &mut Map<String, Json>,
) {
    let mut reg = Handlebars::new();
    for doc in docs {
        reg.register_template_string(doc.file_name, doc.document)
            .unwrap();

        let render = reg.render(doc.file_name, &data).unwrap();
        let mut path = PathBuf::from(path);
        path.push(doc.file_name);
        write_to_file(&*path, render).unwrap();
    }
}

fn write_to_file(path: &path::Path, contents: String) -> Result<(), io::Error> {
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
