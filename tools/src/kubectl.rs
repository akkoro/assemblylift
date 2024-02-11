use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// use itertools::Itertools;
use serde_json::Value;

use crate::Tool;

pub struct KubeCtl {
    cmd: String,
    path: String,
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
        crate::fetch(&s).unwrap();
        s
    }

    pub fn default_with_config(kubeconfig: String) -> Self {
        Self::new("kubectl", ".asml/bin", Some(kubeconfig))
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
        let kubeconfig = match &self.kubeconfig {
            Some(cfg) => vec![format!("--kubeconfig={}", cfg)],
            None => Vec::default(),
        };
        let child = self
            .command()
            .args(kubeconfig)
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
        return "https://dl.k8s.io/release/v1.28.2/bin/linux/amd64/kubectl";
        #[cfg(target_os = "macos")]
        return "https://dl.k8s.io/release/v1.28.2/bin/darwin/amd64/kubectl";
    }
}
