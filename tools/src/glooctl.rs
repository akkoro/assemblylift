use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Tool;

pub struct GlooCtl {
    cmd: String,
    path: String,
    kubeconfig: Option<String>,
}

impl Default for GlooCtl {
    fn default() -> Self {
        GlooCtl::new("glooctl", ".asml/bin", None)
    }
}

impl GlooCtl {
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
        Self::new("glooctl", ".asml/bin", Some(kubeconfig))
    }

    pub fn install_gateway(&self) {
        println!("Installing Gloo API Gateway");
        let mut args = vec!["install", "gateway"];
        if let Some(cfg_path) = &self.kubeconfig {
            args.append(&mut vec!["--kubeconfig", &cfg_path]);
        }
        self.command()
            .args(args)
            .output()
            .expect("glooctl could not install gloo gateway");
    }

    #[allow(dead_code)]
    pub fn uninstall_gateway(&self) {
        println!("Uninstalling Gloo API Gateway");
        self.command()
            .args(vec!["uninstall", "gateway"])
            .output()
            .expect("glooctl could not uninstall gloo gateway");
    }
}

impl Tool for GlooCtl {
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
        return "https://github.com/solo-io/gloo/releases/download/v1.15.18/glooctl-linux-amd64";
        #[cfg(target_os = "macos")]
        return "https://github.com/solo-io/gloo/releases/download/v1.15.18/glooctl-darwin-amd64";
    }
}
