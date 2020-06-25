use crate::iomod::registry::ModuleRegistry;

pub struct IoModulePlugin {
    pub rustc_version: &'static str,
    pub asml_core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut ModuleRegistry),
}
