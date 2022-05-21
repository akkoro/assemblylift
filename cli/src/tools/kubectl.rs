use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::tools::Tool;

pub struct KubeCtl {
    cmd: String,
    path: String,
    // TODO kubeconfig
}

impl Default for KubeCtl {
    fn default() -> Self {
        KubeCtl::new("kubectl", ".asml/bin")
    }
}

impl KubeCtl {
    pub fn new(name: &str, path: &str) -> Self {
        let s = Self {
            cmd: name.into(),
            path: path.into(),
        };
        crate::tools::fetch(&s).unwrap();
        s
    }

    pub fn apply(&self, config: &str) -> Result<Value, String> {
        println!("DEBUG applying config {:?}", config);
        let mut child = self
            .command()
            .arg("apply")
            .arg("-f")
            .arg("-")
            .arg("-o")
            .arg("json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(config.as_bytes()).expect("Failed to write to stdin");
        let output = child.wait_with_output().unwrap();
        let json = std::str::from_utf8(&*output.stdout).unwrap();
        Ok(serde_json::from_str(json).unwrap())
    }
}

impl Tool for KubeCtl {
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
        return "https://dl.k8s.io/release/v1.22.5/bin/linux/amd64/kubectl";
        #[cfg(target_os = "macos")]
        return "https://dl.k8s.io/release/v1.22.5/bin/darwin/amd64/kubectl";
    }
}
