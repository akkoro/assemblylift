use std::process;

use clap::ArgMatches;

pub fn init(matches: Option<&ArgMatches>) {
    if let Some(matches) = matches {
        match matches.value_of("language") {
            Some("rust") => {
                assert_prereqs();
                // TODO generate rust project
            }
            Some(unknown) => panic!("unsupported language: {}", unknown),
            _ => {}
        }
    }
}

fn check_prereqs() -> bool {
    let cargo_version = process::Command::new("cargo")
        .arg("--version")
        .output();

    match cargo_version {
        Ok(version) => {
            println!("Found Cargo: {}", String::from_utf8_lossy(&version.stdout));
            true
        },
        Err(_) => {
            println!("ERROR: missing Cargo!");
            false
        }
    }
}

fn assert_prereqs() {
    if !check_prereqs() {
        panic!("missing system dependencies")
    }
}
