use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::Tool;

pub struct Terraform {
    cmd: String,
    path: String,
}

impl Default for Terraform {
    fn default() -> Self {
        Terraform::new("terraform", ".asml/bin")
    }
}

impl Terraform {
    pub fn new(name: &str, path: &str) -> Self {
        let s = Self {
            cmd: name.into(),
            path: path.into(),
        };
        crate::fetch(&s).unwrap();
        s
    }

    pub fn init(&self) {
        let mut terraform_result = self.command()
            .arg("-chdir=./net")
            .arg("init")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        match terraform_result.wait() {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub fn plan(&self) {
        let mut terraform_result = self.command()
            .arg("-chdir=./net")
            .arg("plan")
            .arg("-out=./plan")
            .arg("-state=../terraform.tfstate")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        match terraform_result.wait() {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub fn apply(&self) {
        let mut terraform_result = self.command()
            .arg("-chdir=./net")
            .arg("apply")
            .arg("-state=../terraform.tfstate")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        match terraform_result.wait() {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub fn destroy(&self) {
        let mut terraform_result = self.command()
            .arg("-chdir=./net")
            .arg("destroy")
            .arg("-state=../terraform.tfstate")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        match terraform_result.wait() {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

impl Tool for Terraform {
    fn command_name(&self) -> &str {
        self.cmd.as_str()
    }

    fn command_path(&self) -> PathBuf {
        Path::new(&format!("{}/{}", self.path, self.cmd)).into()
    }

    fn command(&self) -> Command {
        Command::new(self.command_path())
    }

    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn fetch_url(&self) -> &str {
        #[cfg(target_os = "linux")]
        return "https://releases.hashicorp.com/terraform/1.4.6/terraform_1.4.6_linux_amd64.zip";
        #[cfg(target_os = "macos")]
        return "https://releases.hashicorp.com/terraform/1.4.6/terraform_1.4.6_darwin_amd64.zip";
        #[cfg(target_os = "freebsd")]
        return "https://releases.hashicorp.com/terraform/1.4.6/terraform_1.4.6_freebsd_amd64.zip";
    }
}
