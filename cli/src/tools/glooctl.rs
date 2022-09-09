use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::tools::kubectl::KubeCtl;
use crate::tools::Tool;

pub struct GlooCtl {
    cmd: String,
    path: String,
    // TODO kubeconfig
}

impl Default for GlooCtl {
    fn default() -> Self {
        GlooCtl::new("glooctl", ".asml/bin")
    }
}

impl GlooCtl {
    pub fn new(name: &str, path: &str) -> Self {
        let s = Self {
            cmd: name.into(),
            path: path.into(),
        };
        crate::tools::fetch(&s).unwrap();
        s
    }

    pub fn install_gateway(&self) {
        println!("Installing Gloo API Gateway");
        self.command()
            .args(vec!["install", "gateway"])
            .output()
            .expect("glooctl could not install gloo gateway");
    }

    pub fn uninstall_gateway(&self) {
        println!("Uninstalling Gloo API Gateway");
        self.command()
            .args(vec!["uninstall", "gateway"])
            .output()
            .expect("glooctl could not uninstall gloo gateway");
    }

    pub fn get_upstreams(&self) -> Value {
        let out = self
            .command()
            .arg("get")
            .arg("upstreams")
            .arg("-o")
            .arg("json")
            .output()
            .unwrap();
        let json = std::str::from_utf8(&out.stdout).unwrap();
        serde_json::from_str(json).unwrap()
    }

    pub fn get_virtualservice(&self, vs: &str, ns: Option<&str>) -> Result<Value, String> {
        let kube = KubeCtl::default();
        let out = kube
            .command()
            .arg("get")
            .arg("virtualservice")
            .arg(vs)
            .arg("-n")
            .arg(ns.unwrap_or("gloo-system"))
            .arg("-o")
            .arg("json")
            .output()
            .unwrap();
        let json = std::str::from_utf8(&out.stdout).unwrap();
        let value: Value = serde_json::from_str(json).unwrap();
        match value.get("apiVersion").unwrap().as_str().unwrap() {
            "gateway.solo.io/v1" => Ok(value),
            _ => Err(format!("`{}` is not a gloo virtualservice", vs)),
        }
    }

    pub fn get_upstream(&self, us: &str, ns: Option<&str>) -> Result<Value, String> {
        let kube = KubeCtl::default();
        let out = kube
            .command()
            .arg("get")
            .arg("upstream")
            .arg(us)
            .arg("-n")
            .arg(ns.unwrap_or("gloo-system"))
            .arg("-o")
            .arg("json")
            .output()
            .unwrap();
        let json = std::str::from_utf8(&out.stdout).unwrap();
        let value: Value = serde_json::from_str(json).unwrap();
        match value.get("apiVersion").unwrap().as_str().unwrap() {
            "gateway.solo.io/v1" => Ok(value),
            _ => Err(format!("`{}` is not a gloo upstream", us)),
        }
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
        return "https://github.com/solo-io/gloo/releases/download/v1.11.13/glooctl-linux-amd64";
        #[cfg(target_os = "macos")]
        return "https://github.com/solo-io/gloo/releases/download/v1.11.13/glooctl-darwin-amd64";
    }
}
