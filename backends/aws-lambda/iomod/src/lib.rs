#[macro_use]
extern crate lazy_static;

use std::ffi::c_void;
use std::sync::Mutex;

pub mod database {
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::HashMap;
    use std::ffi::c_void;
    use std::ops::{DerefMut, Deref};
    use std::sync::{Arc, Mutex};

    use crossbeam_utils::atomic::AtomicCell;
    use futures::TryFutureExt;
    use rusoto_core::Region;
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesError, ListTablesInput, ListTablesOutput};
    use wasmer_runtime::{Ctx, Func, Instance};
    use wasmer_runtime_core::vm;

    use assemblylift_core::WasmBufferPtr;
    use assemblylift_core::iomod::{IoModule, ModuleRegistry, AsmlAbiFn};
    use assemblylift_core_event::*;
    use assemblylift_core_event::constants::EVENT_BUFFER_SIZE_BYTES;
    use assemblylift_core_event::threader::Threader;

    lazy_static! {
        static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    }

    async fn __aws_dynamodb_list_tables_impl() -> Vec<u8> {
        println!("TRACE: __aws_dynamodb_list_tables_impl");

        let result = DYNAMODB.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None,
        }).await.unwrap();

        bincode::serialize(&result).unwrap()
    }

    pub fn aws_dynamodb_list_tables_impl(ctx: &mut vm::Ctx, mem: WasmBufferPtr) -> i32 {
        println!("TRACE: Called aws_dynamodb_list_tables_impl");

        let mut threader: *mut Threader = ctx.data.cast();
        let mut threader_ref = unsafe { threader.as_mut().unwrap() };

        let event_id = threader_ref.next_event_id().unwrap();
        println!("DEBUG: event_id={}", event_id);

        println!("TRACE: building wasm memory writer");
        let wasm_instance_memory = ctx.memory(0);
        let mut memory_writer: &[AtomicCell<u8>] = mem
            .deref(wasm_instance_memory, 0, EVENT_BUFFER_SIZE_BYTES as u32)
            .unwrap();

        println!("TRACE: spawning event for aws_dynamodb_list_tables_impl");
        threader_ref.spawn_with_event_id(memory_writer.as_ptr(), __aws_dynamodb_list_tables_impl(), event_id);

        event_id as i32
    }

    pub struct MyModule {}

    impl IoModule for MyModule {
        fn register(registry: &mut ModuleRegistry) {
            // TODO a lot of this can be hidden by a macro

            let org = "aws".to_string();
            let namespace = "dynamodb".to_string();
            let list_tables_name = "list_tables".to_string();

            let mut name_map = HashMap::new();
            name_map.entry(list_tables_name).or_insert(aws_dynamodb_list_tables_impl as AsmlAbiFn);

            let mut namespace_map = HashMap::new();
            namespace_map.entry(namespace).or_insert(name_map);

            registry.modules.entry(org).or_insert(namespace_map);
        }
    }

}
