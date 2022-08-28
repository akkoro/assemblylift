use clap::ArgMatches;

use assemblylift_core_iomod::package::IomodManifest;

use crate::archive;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    match matches.subcommand() {
        ("iomod", matches) => command_iomod(matches),
        _ => println!(
            "{}",
            "missing subcommand. try `asml pack help` for options."
        ),
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

    let manifest = IomodManifest::read(&manifest_path).expect(&format!(
        "could not read iomod manifest from {:?}",
        manifest_path
    ));

    let entrypoint = manifest.process.entrypoint;
    let mut binary_path = cwd.clone();
    binary_path.push(entrypoint);

    // verify that the entrypoint exists before we pack it
    std::fs::metadata(binary_path.clone())
        .expect(&format!("could not stat {:?}", binary_path.clone()));

    let out_path = matches.value_of("out").unwrap(); // unwrap: this arg is required

    archive::zip_dirs(vec![cwd], out_path, vec![]).expect("zip_dir failed during pack");
}
