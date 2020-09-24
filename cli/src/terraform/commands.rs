use std::process;
use std::process::Stdio;

use crate::terraform::relative_binary_path;

pub fn init() {
    let mut terraform_result = process::Command::new(relative_binary_path())
        .arg("init")
        .arg("./net")
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
        .arg("plan")
        .arg("-out=./net/plan")
        .arg("./net")
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
        .arg("apply")
        .arg("./net/plan")
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
        .arg("destroy")
        .arg("./net")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}
