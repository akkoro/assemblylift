use crate::iomod::registry::ModuleRegistry;
use std::ffi::OsStr;
use std::io;
use std::rc::Rc;
use libloading::Library;
use crate::iomod::macros::{RUSTC_VERSION, CORE_VERSION};

pub struct IoModulePlugin {
    pub rustc_version: &'static str,
    pub asml_core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut ModuleRegistry),
}

pub unsafe fn load<P: AsRef<OsStr>>(
    registry: &mut ModuleRegistry,
    library_path: P,
) -> io::Result<()> {
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

    println!("TRACE: loaded IOmod from plugin");

    Ok(())
}
