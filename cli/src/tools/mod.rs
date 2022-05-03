use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

pub mod glooctl;
pub mod kubectl;

pub trait Tool {
    fn command_name(&self) -> &str;
    fn command_path(&self) -> PathBuf;
    fn command(&self) -> Command;
    fn path(&self) -> &str;
    fn fetch_url(&self) -> &str;
}

pub fn fetch<T>(tool: &T) -> Result<(), String>
where
    T: Tool + Sized,
{
    if !tool.command_path().exists() {
        println!("DEBUG fetching tool {}", tool.command_name());
        let mut response = reqwest::blocking::get(tool.fetch_url())
            .expect(&*format!("could not download {}", tool.command_name()));
        if !response.status().is_success() {
            panic!("unable to fetch {} from {}", tool.command_name(), tool.fetch_url());
        }
        let mut response_buffer = Vec::new();
        response.read_to_end(&mut response_buffer).unwrap();

        std::fs::create_dir_all(tool.path().clone()).unwrap();
        std::fs::write(tool.command_path(), response_buffer).unwrap();

        let mut perms = std::fs::metadata(tool.command_path()).unwrap().permissions();
        perms.set_mode(0o755);
        if let Err(_) = std::fs::set_permissions(tool.command_path(), perms) {
            panic!("could not set {:?} binary executable (octal 755) permissions", tool.command_path())
        }
    }
    // TODO handle errors
    Ok(())
}
