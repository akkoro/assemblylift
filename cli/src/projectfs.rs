use std::path::{Path, PathBuf};
use std::{fs, io};

use path_abs::{PathAbs, PathDir};

pub struct Project {
    project_path: Box<PathBuf>,
    service_path: Box<PathBuf>,
}

pub struct ServiceDir {
    dir: Box<PathBuf>,
}

impl ServiceDir {
    pub fn new(dir: Box<PathBuf>) -> Self {
        ServiceDir { dir }
    }

    pub fn dir(&self) -> Box<PathBuf> {
        self.dir.clone()
    }

    pub fn function_dir(&self, name: String) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}",
            self.dir.clone().into_os_string().into_string().unwrap(),
            name
        ))
    }
}

impl Project {
    pub fn new(name: String, project_path: Option<PathBuf>) -> Self {
        let project_path = match project_path {
            Some(path) => {
                if !Path::exists(&*path.clone()) {
                    fs::create_dir(path.clone()).expect(&*format!(
                        "could not create dir {}",
                        path.clone().into_os_string().into_string().unwrap()
                    ));
                }
                Box::new(PathBuf::from(
                    PathAbs::from(
                        PathDir::new(path.clone())
                            .expect(&*format!("couldn't make PathDir for {:?}", path.clone())),
                    )
                    .as_path(),
                ))
            }

            None => {
                let path = format!("./{}", name);
                if !Path::exists(path.as_ref()) {
                    fs::create_dir(path.clone())
                        .expect(&*format!("could not create dir {}", path.clone()));
                }
                Box::new(PathBuf::from(
                    PathAbs::from(PathDir::new(path.clone()).unwrap()).as_path(),
                ))
            }
        };

        let path = format!(
            "{}/services",
            project_path.clone().into_os_string().into_string().unwrap()
        );
        if !Path::exists(path.as_ref()) {
            fs::create_dir(path.clone()).expect(&*format!("could not create dir {}", path.clone()));
        }
        let service_path = Box::new(PathBuf::from(
            PathAbs::from(PathDir::new(path.clone()).unwrap()).as_path(),
        ));

        Self {
            project_path,
            service_path,
        }
    }

    pub fn init(
        &self,
        default_service_name: &str,
        default_function_name: &str,
    ) -> Result<(), io::Error> {
        fs::create_dir_all(format!(
            "{}/{}/{}/src",
            self.service_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
            default_service_name,
            default_function_name
        ))?;
        fs::create_dir_all(format!(
            "{}/{}/{}/.cargo",
            self.service_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
            default_service_name,
            default_function_name
        ))?;

        Ok(())
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
        ServiceDir::new(Box::new(path))
    }

    pub fn dir(&self) -> Box<PathBuf> {
        self.project_path.clone()
    }
}
