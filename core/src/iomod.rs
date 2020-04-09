use wasmer_runtime::Ctx;
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime_core::vm;
use std::io;
use std::cell::Cell;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::error::Error;
use std::borrow::Borrow;
use std::sync::Mutex;
use crate::InstanceData;

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

pub trait IoModule {
    fn register(registry: &mut ModuleRegistry); // MAYBE
}

pub struct ModuleRegistry {
    pub modules: HashMap<String, HashMap<String, HashMap<String, fn(&mut vm::Ctx)->i32>>>
}

pub fn asml_abi_invoke(ctx: &mut vm::Ctx, ptr: u32, len: u32) -> i32 {
    println!("asml_abi_invoke called");
    if let Ok(coords) = ctx_ptr_to_string(ctx, ptr, len) {
        let coord_vec = coords.split(".").collect::<Vec<&str>>();
        let org = coord_vec[0];
        let namespace = coord_vec[1];
        let name = coord_vec[2];
        println!("  with coordinates: {:?}", coord_vec);

        let mut instance_data: &mut InstanceData;
        unsafe {
            instance_data = *ctx.data.cast::<&mut InstanceData>();
        }

        // MUSTDO assert instance_data is valid

        return instance_data.module_registry.modules[org][namespace][name](ctx);
    }

    println!("asml_abi_invoke error");
    -1i32 // error
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
