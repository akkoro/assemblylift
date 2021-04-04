use std::{io, path::PathBuf};

use clap::ArgMatches;
use serde::Deserialize;

use crate::archive;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    match matches.subcommand() {
        ("iomod", matches) => command_iomod(matches),
        _ => println!("{}", "missing subcommand. try `asml pack help` for options."),
    }
}

fn command_iomod(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    let cwd = std::env::current_dir().expect("unable to determine the current working directory");
    let mut manifest_path = cwd.clone();
    manifest_path.push("iomod.toml");

    let manifest = IomodManifest::read(&manifest_path)
        .expect(&format!("could not read iomod manifest from {:?}", manifest_path));

    let entrypoint = manifest.process.entrypoint;
    let mut binary_path = cwd.clone();
    binary_path.push(entrypoint);
    
    // verify that the entrypoint exists before we pack it
    std::fs::metadata(binary_path.clone())
        .expect(&format!("could not stat {:?}", binary_path.clone()));

    let out_path = matches.value_of("out").unwrap(); // unwrap: this arg is required

    archive::zip_dir(cwd, out_path).expect("zip_dir failed during pack");
}

#[derive(Deserialize)]
pub struct IomodManifest {
    pub iomod: ManifestHeader,
    pub process: Process,
}

impl IomodManifest {
    pub fn read(path: &PathBuf) -> Result<Self, io::Error> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Ok(Self::from(contents)),
            Err(why) => Err(io::Error::new(io::ErrorKind::Other, why.to_string())),
        }
    }
}

impl From<String> for IomodManifest {
    fn from(string: String) -> Self {
        match toml::from_str(&string) {
            Ok(manifest) => manifest,
            Err(why) => panic!("error parsing ServiceManifest: {}", why.to_string()),
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
   entrypoint: String,
   arguments: Option<Vec<String>>,
}
