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

pub fn load<P: AsRef<OsStr>>(
    mut registry: &mut ModuleRegistry,
    library_path: P,
) -> io::Result<IoModulePlugin> {
    let library = Rc::new(Library::new(library_path).unwrap());

    let decl: IoModulePlugin;
    unsafe {
        decl = library
            .get::<*mut IoModulePlugin>(b"__ASML_IOMOD_PLUGIN_DECL\0")
            .unwrap()
            .read();
    }

    println!("DEBUG: IOmod compiled with rustc {}", decl.rustc_version);
    println!("DEBUG: IOmod compiled with asml core {}", decl.asml_core_version);
    println!("DEBUG: IOmod register handle addr {:p}", decl.register as  *const ());

    if decl.rustc_version != RUSTC_VERSION || decl.asml_core_version != CORE_VERSION {
        return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
    }

    println!("TRACE: registering IOmod {}", decl.name);

    unsafe { (decl.register)(&mut registry) }

    Ok(decl)
}
