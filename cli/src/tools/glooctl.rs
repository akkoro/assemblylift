use std::borrow::BorrowMut;
use std::iter::Map;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

use serde::{Deserialize, Serialize};
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

    // pub fn add_route(&self, path: &str, upstream_name: &str) -> Result<Value, String> {
    //     let out = self
    //         .command()
    //         .arg("add")
    //         .arg("route")
    //         .arg("--path-exact")
    //         .arg(path)
    //         .arg("--dest-name")
    //         .arg(upstream_name)
    //         .arg("-o")
    //         .arg("json")
    //         .output()
    //         .unwrap();
    //     let json = std::str::from_utf8(&out.stdout).unwrap();
    //     Ok(serde_json::from_str(json).unwrap())
    // }
    //
    // pub fn remove_route(&self, _path: &str, function_coordinates: Vec<&str>) -> Result<(), String> {
    //     let val = self.get_virtualservice("default", None).unwrap();
    //     let mut vs: VirtualService = serde_json::from_value(val).unwrap();
    //
    //     vs.spec.virtual_host.routes.retain(|r| {
    //         r.route_action.single.upstream.name
    //             != format!(
    //                 "asml-{}-{}-asml-{}-{}",
    //                 function_coordinates[0],
    //                 function_coordinates[1],
    //                 function_coordinates[1],
    //                 function_coordinates[2]
    //             )
    //     });
    //
    //     // println!("vs = {:?}", vs);
    //
    //     let kube = KubeCtl::default();
    //     let res = kube.apply(&*serde_json::to_string(&vs).unwrap()).unwrap();
    //     println!("res = {:?}", res);
    //
    //     Ok(())
    // }

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
        return "https://github.com/solo-io/gloo/releases/download/v1.12.0-beta7/glooctl-linux-amd64";
        #[cfg(target_os = "macos")]
        return "https://github.com/solo-io/gloo/releases/download/v1.12.0-beta7/glooctl-darwin-amd64";
    }
}


