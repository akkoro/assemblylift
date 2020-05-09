use std::borrow::{Borrow, BorrowMut};
use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::sync::{Mutex, Arc};
use std::task::Context;

use wasmer_runtime::Ctx;
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime_core::{DynFunc, structures::TypedIndex, types::TableIndex, vm};

use crate::InstanceData;
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

#[derive(Clone)]
pub struct ModuleRegistry {
    pub modules: HashMap<String, HashMap<String, HashMap<String, fn(&mut vm::Ctx)->i32>>>
}

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry {
            modules: Default::default()
        }
    }
}

pub fn asml_abi_invoke(ctx: &mut vm::Ctx, ptr: u32, len: u32) -> i32 {
    println!("TRACE: asml_abi_invoke called");
    if let Ok(coords) = ctx_ptr_to_string(ctx, ptr, len) {
        let coord_vec = coords.split(".").collect::<Vec<&str>>();
        let org = coord_vec[0];
        let namespace = coord_vec[1];
        let name = coord_vec[2];
        println!("  with coordinates: {:?}", coord_vec);

        // MUSTDO assert instance_data is valid

        return MODULE_REGISTRY.lock().unwrap().modules[org][namespace][name](ctx);
    }

    println!("ERROR: asml_abi_invoke error");
    -1i32 // error
}

pub fn asml_abi_poll(ctx: &mut vm::Ctx, id: u32) -> i32 {
    let mut instance_data = get_instance_data(ctx);
    let mut threader = get_threader(instance_data);
    threader.is_event_ready(id) as i32
}

pub fn asml_abi_event_ptr(ctx: &mut vm::Ctx, id: u32) -> u32 {
    let mut instance_data = get_instance_data(ctx);
    let mut threader = get_threader(instance_data);
    threader.get_event_memory_document(id).unwrap().start as u32
}

pub fn asml_abi_event_len(ctx: &mut vm::Ctx, id: u32) -> u32 {
    let mut instance_data = get_instance_data(ctx);
    let mut threader = get_threader(instance_data);
    threader.get_event_memory_document(id).unwrap().length as u32
}

#[inline]
fn get_instance_data(ctx: &mut vm::Ctx) -> &mut InstanceData {
    // TODO check null and return option or result
    let mut instance_data: &mut InstanceData;
    unsafe {
        instance_data = *ctx.data.cast::<&mut InstanceData>();
    }

    instance_data
}

#[inline]
fn get_threader(instance_data: &mut InstanceData) -> &mut Threader {
    // TODO check null and return option or result
    let mut threader: *mut Threader;
    unsafe {
        threader = instance_data.threader;
        threader.as_mut().unwrap()
    }
}

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
