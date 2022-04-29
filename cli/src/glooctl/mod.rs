use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

use serde_json::Value;

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

        if !s.path().exists() {
            let gloo_version = "v1.12.0-beta7";
            #[cfg(target_os = "linux")]
                let runtime_url = &*format!(
                "https://github.com/solo-io/gloo/releases/download/{}/glooctl-linux-amd64",
                gloo_version,
            );
            #[cfg(target_os = "macos")]
                let runtime_url = &*format!(
                "https://github.com/solo-io/gloo/releases/download/{}/glooctl-darwin-amd64",
                gloo_version,
            );
            let mut response = reqwest::blocking::get(runtime_url)
                .expect("could not download glooctl");
            if !response.status().is_success() {
                panic!("unable to fetch glooctl from {}", runtime_url);
            }
            let mut response_buffer = Vec::new();
            response.read_to_end(&mut response_buffer).unwrap();

            std::fs::create_dir_all("./.asml").unwrap();
            std::fs::write("./.asml/bin/glooctl", response_buffer).unwrap();

            let mut perms = std::fs::metadata("./.asml/bin/glooctl").unwrap().permissions();
            perms.set_mode(0o755);
            if let Err(_) = std::fs::set_permissions("./.asml/bin/glooctl", perms) {
                panic!("could not set glooctl binary executable (octal 755) permissions")
            }
        }

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

    pub fn add_route(&self, path: &str, upstream_name: &str) -> Result<Value, String> {
        let out = self
            .command()
            .arg("add")
            .arg("route")
            .arg("--path-exact")
            .arg(path)
            .arg("--dest-name")
            .arg(upstream_name)
            .arg("-o")
            .arg("json")
            .output()
            .unwrap();
        let json = std::str::from_utf8(&out.stdout).unwrap();
        Ok(serde_json::from_str(json).unwrap())
    }

    pub fn path(&self) -> PathBuf {
        Path::new(&format!("{}/{}", self.path, self.cmd))
            .canonicalize()
            .unwrap()
    }

    pub fn command(&self) -> Command {
        process::Command::new(self.path())
    }
}
