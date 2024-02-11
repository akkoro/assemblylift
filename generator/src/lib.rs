use std::{fs::File, io::Write, path::PathBuf};

use handlebars::handlebars_helper;

pub mod context;
pub mod projectfs;
pub mod providers;
pub mod toml;

handlebars_helper!(concat: |s1: String, s2: String| format!("{}{}", s1, s2));
handlebars_helper!(snake_case: |s: String| s.replace("-", "_").replace(".", "_"));

pub type CastResult<T> = std::result::Result<T, CastError>;
pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;
pub type Options = StringMap<String>;

#[derive(Debug)]
pub struct CastError(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    HCL,
    Dockerfile,
}

/// A `Fragment` is the output of a `cast` operation. Its contents may be part of, or the entirety of,
/// some document which will be output at `write_path`.
#[derive(Debug, Clone)]
pub struct Fragment {
    pub content_type: ContentType,
    pub content: String,
    pub write_path: PathBuf,
}

impl Fragment {
    pub fn write(&self) -> Result<(), CastError> {
        let prefix = self
            .write_path
            .parent()
            .ok_or(CastError("write_path has no parent".into()))?;
        std::fs::create_dir_all(prefix).map_err(|e| {
            CastError(format!(
                "could not create directories {}: {}",
                prefix.to_string_lossy(),
                e.to_string()
            ))
        })?;
        let mut fout = File::create(&self.write_path).map_err(|e| {
            CastError(format!(
                "could not create file {}: {}",
                self.write_path.to_string_lossy(),
                e.to_string()
            ))
        })?;
        fout.write_all(self.content.as_bytes())
            .map_err(|e| CastError(format!("could not write contents: {}", e.to_string())))?;
        println!("📄 > Wrote {}", self.write_path.to_string_lossy());

        Ok(())
    }
}

pub fn concat_cast<T>(accum: CastResult<Vec<T>>, v: CastResult<Vec<T>>) -> CastResult<Vec<T>> {
    let mut out = Vec::new();
    out.append(&mut accum?);
    out.append(&mut v?);
    Ok(out)
}
