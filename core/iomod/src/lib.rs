#[macro_use]
extern crate lazy_static;

use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::sync::Mutex;

use crossbeam_utils::atomic::AtomicCell;
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::Ctx;
use wasmer_runtime_core::vm;

use assemblylift_core_event_common::constants::EVENT_BUFFER_SIZE_BYTES;

pub mod macros;
pub mod plugin;

lazy_static! {
    pub static ref MODULE_REGISTRY: Mutex<ModuleRegistry> = Mutex::new(ModuleRegistry::new());
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

pub trait IoModule {
    fn register(registry: &mut ModuleRegistry);
}

pub fn asml_abi_invoke(
    ctx: &mut vm::Ctx,
    mem: WasmBufferPtr,
    name_ptr: u32,
    name_len: u32,
    input: WasmBufferPtr,
    input_len: u32,
) -> i32 {
    println!("TRACE: asml_abi_invoke called");
    unsafe {
        if let Ok(coords) = ctx_ptr_to_string(ctx, name_ptr, name_len) {
            let coord_vec = coords.split(".").collect::<Vec<&str>>();
            let org = coord_vec[0];
            let namespace = coord_vec[1];
            let name = coord_vec[2];
            println!("  with coordinates: {:?}", coord_vec);

            println!("DEBUG: input_len={}", input_len);
            let registry = MODULE_REGISTRY.lock().unwrap();
            return registry.modules[org][namespace][name].0(ctx, mem, input, input_len);
        }
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
    unsafe {
        threader
            .as_mut()
            .unwrap()
            .get_event_memory_document(id)
            .unwrap()
            .start as u32
    }
}

pub fn asml_abi_event_len(ctx: &mut vm::Ctx, id: u32) -> u32 {
    let threader = get_threader(ctx);
    unsafe {
        threader
            .as_mut()
            .unwrap()
            .get_event_memory_document(id)
            .unwrap()
            .length as u32
    }
}

#[inline]
fn get_threader(ctx: &mut vm::Ctx) -> *mut Threader {
    let threader: *mut Threader = ctx.data.cast();
    if threader.is_null() {
        panic!("Threader instance is NULL in asml_abi_poll")
    }

    threader
}

#[inline]
fn ctx_ptr_to_string(ctx: &mut Ctx, ptr: u32, len: u32) -> Result<String, io::Error> {
    let memory = ctx.memory(0);
    let view: MemoryView<u8> = memory.view();

    let mut str_vec: Vec<u8> = Vec::new();
    for byte in view[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(Cell::get)
    {
        str_vec.push(byte);
    }

    std::str::from_utf8(str_vec.as_slice())
        .map(String::from)
        .map_err(to_io_error)
}

// spawn_event was here
