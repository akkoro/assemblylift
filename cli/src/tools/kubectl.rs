use std::path::{Path, PathBuf};
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

    pub fn apply(&self) -> Result<(), String> {
        println!("Applying kubernetes configuration...");
        let mut child = self
            .command()
            .arg("apply")
            .arg("-f")
            .arg("./net/kube.yaml")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let output = child.wait_with_output().unwrap();
        if output.status.code().unwrap() != 0i32 {
            return Err(format!("kubectl apply exited with error code {:?}", output.status))
        }

        let out = std::str::from_utf8(&*output.stdout).unwrap();
        println!("{}", out);
        println!("✅ kubectl apply OK");
        Ok(())
    }

    pub fn get_namespaces(&self) -> Result<Value, String> {
        let mut child = self
            .command()
            .args(vec!["get", "namespaces"])
            .arg("-o")
            .arg("json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
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
