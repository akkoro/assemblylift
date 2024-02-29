use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use path_abs::{PathAbs, PathDir};
use serde::{Deserialize, Serialize};

use super::toml;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    project_path: PathBuf,
    service_path: PathBuf,
}

pub struct ServiceDir {
    dir: PathBuf, // FIXME why is this boxed??
}

impl ServiceDir {
    pub fn new(dir: PathBuf) -> Self {
        ServiceDir { dir }
    }

    pub fn dir(&self) -> PathBuf {
        self.dir.clone()
    }

    pub fn function_dir(&self, name: String) -> PathBuf {
        PathBuf::from(format!(
            "{}/functions/{}",
            self.dir.clone().into_os_string().into_string().unwrap(),
            name
        ))
    }
}

pub struct NetDir {
    dir: PathBuf,
}

impl NetDir {
    pub fn new(project_path: PathBuf) -> Self {
        let mut net_path = project_path;
        net_path.push("net");
        Self { dir: net_path }
    }

    pub fn service_dir(&self, name: &str) -> ServiceDir {
        let path = PathBuf::from(&*format!(
            "{}/services/{}",
            self.dir.clone().into_os_string().into_string().unwrap(),
            name
        ));
        ServiceDir::new(path)
    }

    pub fn runtime_dir(&self) -> PathBuf {
        PathBuf::from(&*format!(
            "{}/runtime",
            self.dir.clone().into_os_string().into_string().unwrap(),
        ))
    }
}

impl Project {
    pub fn new(name: String, project_path: Option<PathBuf>) -> Rc<Self> {
        let project_path = match project_path {
            Some(path) => {
                if !Path::exists(&*path.clone()) {
                    fs::create_dir(path.clone()).expect(&*format!(
                        "could not create dir {}",
                        path.clone().into_os_string().into_string().unwrap()
                    ));
                }
                PathBuf::from(
                    PathAbs::from(
                        PathDir::new(path.clone())
                            .expect(&*format!("couldn't make PathDir for {:?}", path.clone())),
                    )
                    .as_path(),
                )
            }

            None => {
                let path = format!("./{}", name);
                if !Path::exists(path.as_ref()) {
                    fs::create_dir(path.clone())
                        .expect(&*format!("could not create dir {}", path.clone()));
                }
                PathBuf::from(
                    PathAbs::from(PathDir::new(path.clone()).unwrap()).as_path(),
                )
            }
        };

        let path = format!(
            "{}/services",
            project_path.clone().into_os_string().into_string().unwrap()
        );
        if !Path::exists(path.as_ref()) {
            fs::create_dir(path.clone()).expect(&*format!("could not create dir {}", path.clone()));
        }
        let service_path = PathBuf::from(
            PathAbs::from(PathDir::new(path.clone()).unwrap()).as_path(),
        );

        Rc::new(Self {
            name,
            project_path,
            service_path,
        })
    }

    pub fn service_dir(&self, name: String) -> ServiceDir {
        let path = PathBuf::from(&*format!(
            "{}/{}",
            self.service_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
            name
        ));
        ServiceDir::new(path)
    }

    pub fn net_dir(&self) -> NetDir {
        NetDir::new(self.project_path.clone())
    }

    pub fn dir(&self) -> PathBuf {
        self.project_path.clone()
    }
}

pub fn locate_asml_manifest() -> Option<(toml::asml::Manifest, PathBuf)> {
    use walkdir::WalkDir;

    let mut path: Option<PathBuf> = None;
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let file = entry.file_name().to_string_lossy();
        if file.eq_ignore_ascii_case("assemblylift.toml") {
            path = Some(PathBuf::from(file.into_owned()));
            break;
        }
    }

    match path {
        Some(path) => {
            let canonical_path = fs::canonicalize(path.clone()).unwrap();
            Some((
                toml::asml::Manifest::read(&PathBuf::from(canonical_path.clone()))
                    .expect("could not read assemblylift.toml"),
                PathBuf::from(canonical_path.clone()),
            ))
        }
        None => None,
    }
}
