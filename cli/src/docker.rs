use std::process;
use std::process::{ExitStatus, Stdio};

pub fn build(tag: &str, dockerfile: &str) -> Result<ExitStatus, std::io::Error> {
    process::Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(tag)
        .arg(".")
        .arg("--file")
        .arg(dockerfile)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("could not build Dockerfile")
        .wait()
}

pub fn push(image: &str) -> Result<ExitStatus, std::io::Error> { 
    process::Command::new("docker")
        .arg("push")
        .arg(image)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("could not spawn docker")
        .wait()
}

pub fn tag(src: &str, dst: &str) -> Result<ExitStatus, std::io::Error> { 
    process::Command::new("docker")
        .arg("tag")
        .arg(src)
        .arg(dst)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("could not spawn docker")
        .wait()
}
