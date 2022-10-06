use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::archive;

pub mod commands;

pub fn relative_binary_path() -> &'static str {
    ".asml/bin/terraform"
}

pub fn fetch(project_path: &PathBuf) {
    use std::io::Read;

    let terraform_path = format!(
        "{}/{}",
        project_path.clone().into_os_string().into_string().unwrap(),
        relative_binary_path()
    );

    if Path::new(&terraform_path).exists() {
        println!(
            "Found terraform at {}, skipping download...",
            terraform_path
        );
        return;
    }

    println!("Extracting terraform to {}", terraform_path);

    let mut terraform_zip = Vec::new();

    #[cfg(target_os = "linux")]
    let mut response = reqwest::blocking::get(
        "https://releases.hashicorp.com/terraform/1.3.2/terraform_1.3.2_linux_amd64.zip",
    )
    .unwrap();
    #[cfg(target_os = "macos")]
    let mut response = reqwest::blocking::get(
        "https://releases.hashicorp.com/terraform/1.3.2/terraform_1.3.2_darwin_amd64.zip",
    )
    .unwrap();

    response.read_to_end(&mut terraform_zip).unwrap();

    if let Err(_) = fs::create_dir_all(terraform_path.replace("/terraform", "")) {
        panic!("could not create directory ./.asml/bin")
    }

    archive::unzip_terraform(terraform_zip, &terraform_path).unwrap();

    let mut perms = fs::metadata(&terraform_path).unwrap().permissions();
    perms.set_mode(0o755);
    if let Err(_) = fs::set_permissions(&terraform_path, perms) {
        panic!("could not set terraform binary executable (octal 755) permissions")
    }
}
