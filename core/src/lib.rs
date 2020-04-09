extern crate assemblylift_core_event;
use assemblylift_core_event::Event;
use assemblylift_core_event::manager::EventManager;

use crate::iomod::ModuleRegistry;

use std::io::Read;

use wasmer_runtime::Instance;
use wasmer_runtime::{Func, Ctx, WasmPtr, Array};

pub mod iomod;

pub struct InstanceData<'a> {
    pub module_registry: &'a mut ModuleRegistry,
    pub event_manager: &'a mut EventManager,
}

/* Cloud interface */

pub trait Database {
    // TODO: general-purpose database api ?
}

// Event SerDe

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

pub unsafe fn serialize_event_from_host(id: usize, event: &Event, ctx: &mut Ctx) {
    let event_size = std::mem::size_of::<Event>();
    let mut idx = id * event_size;
    let buffer = any_as_u8_slice(event);
    for b in buffer {
        if let Some(mut get_buffer) = (*ctx.module)
            .info
            .exports
            .get("__al_get_event_buffer_pointer") {
            // TODO do something with the get_buffer function and hope that it works!
        }

        idx += 1;
    }
}

// pub unsafe fn deserialize_event(id: usize) -> Box<Event> {
//     use crate::event::widget::EVENT_BUFFER;
//
//     let event_size = std::mem::size_of::<Event>();
//     let buffer_size = std::mem::size_of::<EventBuffer>();
//
//     let mut buffer: &[u8] = EVENT_BUFFER[id * event_size..buffer_size].as_ref();
//     let mut event: Event = std::mem::zeroed();
//
//     let event_slice = std::slice::from_raw_parts_mut(&mut event as *mut _ as *mut u8, event_size);
//     buffer.read_exact(event_slice).unwrap();
//
//     Box::from(event)
// }