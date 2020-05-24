use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::sync::Mutex;

use wasmer_runtime::Ctx;
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime_core::vm;

use crate::WasmBufferPtr;
use assemblylift_core_event::threader::Threader;

lazy_static! {
    pub static ref MODULE_REGISTRY: Mutex<ModuleRegistry> = Mutex::new(ModuleRegistry::new());
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

pub trait IoModule {
    fn register(registry: &mut ModuleRegistry); // MAYBE
}

pub type AsmlAbiFn = fn(&mut vm::Ctx, WasmBufferPtr, WasmBufferPtr, u32) -> i32;
pub type ModuleMap = HashMap<String, HashMap<String, HashMap<String, AsmlAbiFn>>>;

#[derive(Clone)]
pub struct ModuleRegistry {
    pub modules: ModuleMap
}

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry {
            modules: Default::default()
        }
    }
}

pub fn asml_abi_invoke(ctx: &mut vm::Ctx, mem: WasmBufferPtr, name_ptr: u32, name_len: u32, input: WasmBufferPtr, input_len: u32) -> i32 {
    println!("TRACE: asml_abi_invoke called");
    if let Ok(coords) = ctx_ptr_to_string(ctx, name_ptr, name_len) {
        let coord_vec = coords.split(".").collect::<Vec<&str>>();
        let org = coord_vec[0];
        let namespace = coord_vec[1];
        let name = coord_vec[2];
        println!("  with coordinates: {:?}", coord_vec);

        println!("DEBUG: input_len={}", input_len);
        let registry = MODULE_REGISTRY.lock().unwrap();
        return registry.modules[org][namespace][name](ctx, mem, input, input_len);
    }

    println!("ERROR: asml_abi_invoke error");
    -1i32 // error
}

pub fn asml_abi_poll(ctx: &mut vm::Ctx, id: u32) -> i32 {
    let threader = get_threader(ctx);
    unsafe { threader.as_mut().unwrap().is_event_ready(id) as i32 }
}

pub fn asml_abi_event_ptr(ctx: &mut vm::Ctx, id: u32) -> u32 {
    let threader = get_threader(ctx);
    unsafe { threader.as_mut().unwrap().get_event_memory_document(id).unwrap().start as u32 }
}

pub fn asml_abi_event_len(ctx: &mut vm::Ctx, id: u32) -> u32 {
    let threader = get_threader(ctx);
    unsafe { threader.as_mut().unwrap().get_event_memory_document(id).unwrap().length as u32 }
}

#[inline]
fn get_threader(ctx: &mut vm::Ctx) -> *mut Threader {
    // TODO check null and return option or result
    let threader: *mut Threader = ctx.data.cast();
    threader
}

#[inline]
fn ctx_ptr_to_string(ctx: &mut Ctx, ptr: u32, len: u32) -> Result<String, io::Error> {
    let memory = ctx.memory(0);
    let view: MemoryView<u8> = memory.view();

    let mut str_vec: Vec<u8> = Vec::new();
    for byte in view[ptr as usize .. (ptr + len) as usize].iter().map(Cell::get) {
        str_vec.push(byte);
    }

    std::str::from_utf8(str_vec.as_slice())
        .map(String::from)
        .map_err(to_io_error)
}

#[macro_export]
macro_rules! register_calls {
    ($reg:expr, $($org_name:ident => {
        $ns_name:ident => $ns:tt
    }),* $(,)?) 
    => {{
        let org_name = String::from("$org_name");
        let ns_name = String::from("$ns_name");

        let mut namespace_map = HashMap::new();

        $({
            let mut name_map = __register_calls!($ns);
            namespace_map.entry(ns_name).or_insert(name_map);
        })*

        $reg.modules.entry(org_name).or_insert(namespace_map);
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_calls {
    ({ $( $call_name:ident => $call:expr ),* $(,)? }) => {{
        let mut name_map = HashMap::new();
        $(
            let call_name = String::from("$call_name");
            name_map.entry(call_name).or_insert($call as AsmlAbiFn);
        )*
        name_map
    }};
}
