use std::ffi::OsStr;
use std::io;
use std::rc::Rc;
use std::sync::Arc;

use libloading::Library;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use assemblylift_core::IoModulePlugin;
use assemblylift_core::registry::ModuleRegistry;

use crate::macros::{CORE_VERSION, RUSTC_VERSION};

pub unsafe fn load<P: AsRef<OsStr>>(
    mut registry: &mut ModuleRegistry,
    library_path: P,
) -> io::Result<IoModulePlugin> {
    let library = Rc::new(Library::new(library_path).unwrap());

    let decl = library
        .get::<*mut IoModulePlugin>(b"__ASML_IOMOD_PLUGIN_DECL\0")
        .unwrap()
        .read();

    if decl.rustc_version != RUSTC_VERSION || decl.asml_core_version != CORE_VERSION {
        return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
    }

    println!("TRACE: loaded IOmod {}", decl.name);

    (decl.register)(&mut registry);

    Ok(decl)
}
