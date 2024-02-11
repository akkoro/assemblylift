use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Tool;

pub struct CmCtl {
    cmd: String,
    path: String,
    kubeconfig: Option<String>,
}

impl Default for CmCtl {
    fn default() -> Self {
        CmCtl::new("cmctl", ".asml/bin", None)
    }
}

impl CmCtl {
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
        Self::new("cmctl", ".asml/bin", Some(kubeconfig))
    }

    pub fn install(&self) {
        println!("Installing cert-manager");
        let kubeconfig = match &self.kubeconfig {
            Some(cfg) => vec!["--kubeconfig", &cfg],
            None => Vec::default(),
        };
        self.command()
            .args(kubeconfig)
            .args(vec!["x", "install"])
            .output()
            .expect("cmctl could not install cert-manager");
    }
}

impl Tool for CmCtl {
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
        return "https://github.com/cert-manager/cert-manager/releases/download/v1.13.3/cmctl-linux-amd64.tar.gz";
        #[cfg(target_os = "macos")]
        return "https://github.com/cert-manager/cert-manager/releases/download/v1.13.3/cmctl-darwin-amd64.tar.gz";
    }
}
