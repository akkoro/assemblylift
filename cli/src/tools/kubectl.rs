use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use itertools::Itertools;
use serde_json::Value;

use crate::tools::Tool;

pub struct KubeCtl {
    cmd: String,
    path: String,
    // TODO pass to args
    kubeconfig: Option<String>,
}

impl Default for KubeCtl {
    fn default() -> Self {
        KubeCtl::new("kubectl", ".asml/bin", None)
    }
}

impl KubeCtl {
    pub fn new(name: &str, path: &str, kubeconfig: Option<String>) -> Self {
        let s = Self {
            cmd: name.into(),
            path: path.into(),
            kubeconfig,
        };
        crate::tools::fetch(&s).unwrap();
        s
    }

    #[allow(dead_code)]
    pub fn apply(&self) -> Result<(), String> {
        println!("Applying kubernetes configuration...");
        let child = self
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
            return Err(format!(
                "kubectl apply exited with error code {:?}",
                output.status
            ));
        }

        let out = std::str::from_utf8(&*output.stdout).unwrap();
        println!("{}", out);
        println!("✅ kubectl apply OK");
        Ok(())
    }

    pub fn apply_from_str(&self, config: &str) -> Result<(), String> {
        println!("Applying kubernetes configuration...");
        let mut child = self
            .command()
            .arg("apply")
            .args(vec!["-f", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let mut stdin = child.stdin.take().unwrap();
            let mut stdin_writer = BufWriter::new(&mut stdin);
            stdin_writer.write(config.as_ref()).unwrap();
        }

        let output = child.wait_with_output().unwrap();
        if output.status.code().unwrap() != 0i32 {
            return Err(format!(
                "kubectl apply exited with error code {:?}",
                output.status
            ));
        }

        let out = std::str::from_utf8(&*output.stdout).unwrap();
        println!("{}", out);
        println!("✅ kubectl apply OK");
        Ok(())
    }

    pub fn get_namespaces(&self) -> Result<Value, String> {
        let child = self
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

    pub fn get(&self, kind: &str) -> Result<Value, String> {
        let child = self
            .command()
            .args(vec!["get", kind])
            .args(vec!["-o", "json"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let output = child.wait_with_output().unwrap();
        let json = std::str::from_utf8(&*output.stdout).unwrap();
        Ok(serde_json::from_str(json).unwrap())
    }

    pub fn get_in_namespace(
        &self,
        kind: &str,
        ns: &str,
        labels: Option<HashMap<String, String>>,
    ) -> Result<Value, String> {
        let label_args = labels
            .unwrap_or(Default::default())
            .into_iter()
            .map(|l| {
                vec![
                    "-l".to_string(),
                    format!("{}={}", l.0.to_owned(), l.1.to_owned()),
                ]
            })
            .reduce(|accum, mut v| {
                let mut a = accum;
                a.append(&mut v);
                a
            })
            .unwrap_or(Default::default());
        let mut child = self
            .command()
            .args(vec!["get", kind])
            .args(vec!["-n", ns])
            .args(label_args)
            .args(vec!["-o", "json"])
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
        return "https://dl.k8s.io/release/v1.27.4/bin/linux/amd64/kubectl";
        #[cfg(target_os = "macos")]
        return "https://dl.k8s.io/release/v1.27.4/bin/darwin/amd64/kubectl";
    }
}
