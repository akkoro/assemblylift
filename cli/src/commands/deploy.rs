use std::fs;
use std::process;
use std::process::Stdio;
use std::os::unix::fs::PermissionsExt;

use clap::ArgMatches;

pub fn deploy(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for deploy command")
    };

    // TODO switch on platform
    // TODO check first if file is present
    let terraform_path = "./.asml/bin/terraform";
    let terraform: &'static [u8] = include_bytes!("../../resources/bin/linux64/terraform");
    if let Err(_) = fs::create_dir_all("./.asml/bin")
        .and_then(|_| fs::write(terraform_path, terraform)) 
    {
        panic!("could not copy terraform binary to ./.asml/bin")
    }

    let mut perms = fs::metadata(terraform_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(terraform_path, perms).unwrap();

    // TODO will probably want to wrap calls to tf in functions
    let mut terraform_result = process::Command::new(terraform_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    
    match terraform_result.wait() {
        Ok(_) => {},
        Err(_) => {}
    }
}
