use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::anyhow;
use flate2::read::GzDecoder;

pub mod cmctl;
pub mod glooctl;
pub mod kubectl;

pub trait Tool {
    fn command_name(&self) -> &str;
    fn command_path(&self) -> PathBuf;
    fn command(&self) -> Command;
    fn path(&self) -> &str;
    fn fetch_url(&self) -> &str;
}

pub fn fetch<T>(tool: &T) -> anyhow::Result<()>
where
    T: Tool + Sized,
{
    if !tool.command_path().exists() {
        println!("🔧  > Fetching tool {}", tool.command_name());

        std::fs::create_dir_all(tool.path().clone()).unwrap();
        let bytes = download_to_bytes(tool.fetch_url())
            .expect(&*format!("could not download {}", tool.command_name()));
        if tool.fetch_url().contains(".tar.gz") {
            // FIXME this leans on the assumption that the only gzipped tool we fetch is cmctl
            let tar = GzDecoder::new(bytes.as_slice());
            let mut ar = tar::Archive::new(tar);
            ar.entries()
                .expect("cmctl archive is empty")
                .find(|e| e.as_ref().unwrap().path().unwrap().file_name().unwrap() == "cmctl")
                .expect("cmctl not found in archive")
                .unwrap()
                .unpack(tool.command_path())
                .expect("could not unpack cmctl");
        } else {
            std::fs::write(tool.command_path(), bytes).unwrap();
        }

        let mut perms = std::fs::metadata(tool.command_path())
            .unwrap()
            .permissions();
        perms.set_mode(0o755);
        if let Err(_) = std::fs::set_permissions(tool.command_path(), perms) {
            panic!(
                "could not set {:?} binary executable (octal 755) permissions",
                tool.command_path()
            )
        }
    }
    // TODO handle errors
    Ok(())
}

pub fn download_to_bytes<T: reqwest::IntoUrl + Clone>(url: T) -> anyhow::Result<Vec<u8>> {
    println!("⏬ > Downloading object from {}...", url.as_str());
    match reqwest::blocking::get(url.clone()) {
        Ok(mut response) => {
            if !response.status().is_success() {
                return Err(anyhow!("unable to download file from {}", url.as_str()));
            }
            let mut response_buffer = Vec::new();
            if let Err(err) = response.read_to_end(&mut response_buffer) {
                return Err(anyhow!(err));
            }

            Ok(response_buffer)
        }
        Err(err) => Err(anyhow!(err)),
    }
}

pub fn download_to_path<T: reqwest::IntoUrl + Clone, P: AsRef<Path>>(
    url: T,
    to: P,
) -> anyhow::Result<()> {
    match download_to_bytes(url) {
        Ok(bytes) => {
            if let Err(err) = std::fs::write(to, bytes) {
                return Err(anyhow!(err));
            }

            Ok(())
        }
        Err(err) => Err(anyhow!(err)),
    }
}
