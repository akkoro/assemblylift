use std::ffi::OsStr;
use std::io;
use std::rc::Rc;

use libloading::Library;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use crate::iomod::macros::{CORE_VERSION, RUSTC_VERSION};
use crate::iomod::registry::ModuleRegistry;
use std::sync::Arc;

pub struct IoModulePlugin {
    pub name: &'static str,
    pub rustc_version: &'static str,
    pub asml_core_version: &'static str,
    pub runtime: Lazy<Arc<Runtime>>,
    pub register: unsafe extern "C" fn(&mut ModuleRegistry, &Runtime),
}

pub unsafe fn load<P: AsRef<OsStr>>(
    mut registry: &mut ModuleRegistry,
    library_path: P,
) -> io::Result<IoModulePlugin> {
    let library = Rc::new(Library::new(library_path).unwrap());

    let decl = library
        .get::<*mut IoModulePlugin>(b"__asml_iomod_plugin_decl\0").unwrap()
        .read();

    if decl.rustc_version != RUSTC_VERSION || decl.asml_core_version != CORE_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Version mismatch",
        ));
    }

    println!("TRACE: loaded IOmod {}", decl.name);

    (decl.register)(&mut registry, &decl.runtime);

    Ok(decl)
}
