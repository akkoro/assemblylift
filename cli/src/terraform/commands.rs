use std::process;
use std::process::Stdio;

use crate::terraform::relative_binary_path;

pub fn init() {
    let mut terraform_result = process::Command::new(relative_binary_path())
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

pub fn plan() {
    let mut terraform_result = process::Command::new(relative_binary_path())
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

pub fn apply() {
    let mut terraform_result = process::Command::new(relative_binary_path())
        .arg("-chdir=./net")
        .arg("apply")
        .arg("-state=../terraform.tfstate") // FIXME does this work if remote state configured?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn destroy() {
    let mut terraform_result = process::Command::new(relative_binary_path())
        .arg("-chdir=./net")
        .arg("destroy")
        .arg("-state=../terraform.tfstate") // FIXME does this work if remote state configured?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}
