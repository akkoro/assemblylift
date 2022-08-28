use serde::Deserialize;

#[derive(Deserialize)]
pub struct IomodManifest {
    pub iomod: ManifestHeader,
    pub process: Process,
}

impl IomodManifest {
    pub fn read(path: &std::path::PathBuf) -> Result<Self, std::io::Error> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Ok(Self::from(contents)),
            Err(why) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                why.to_string(),
            )),
        }
    }
}

impl From<String> for IomodManifest {
    fn from(string: String) -> Self {
        match toml::from_str(&string) {
            Ok(manifest) => manifest,
            Err(why) => panic!("error parsing IomodManifest: {}", why.to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct ManifestHeader {
    pub coordinates: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct Process {
    pub entrypoint: String,
    pub arguments: Option<Vec<String>>,
}
