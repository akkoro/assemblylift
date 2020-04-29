#[macro_use]
extern crate lazy_static;

pub mod database {
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::ffi::c_void;
    use std::ops::DerefMut;
    use std::pin::Pin;
    use std::sync::Mutex;

    use futures::TryFutureExt;
    use rusoto_core::Region;
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesError, ListTablesInput, ListTablesOutput};
    use wasmer_runtime::{Ctx, Func, Instance};
    use wasmer_runtime_core::vm;

    use assemblylift_core::{InstanceData, WasmBufferPtr};
    use assemblylift_core::iomod::{IoModule, ModuleRegistry};
    use assemblylift_core_event::*;
    use assemblylift_core_event::constants::EVENT_BUFFER_SIZE_BYTES;
    use assemblylift_core_event::executor::Executor;
    use assemblylift_core_event::manager::{DynFut, EventManager};

    lazy_static! {
        static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    }

    async fn __aws_dynamodb_list_tables_impl() -> Vec<u8> {
        let result = DYNAMODB.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None,
        }).await.unwrap();

        println!("{:?}", result);
        bincode::serialize(&result).unwrap()
    }

    pub fn aws_dynamodb_list_tables_impl(ctx: &mut vm::Ctx) -> i32 {
        println!("TRACE: Called aws_dynamodb_list_tables_impl");

        let mut instance_data: &mut InstanceData;
        let instance: &Instance;
        unsafe {
            instance_data = *ctx.data.cast::<&mut InstanceData>();
            instance = instance_data.instance.as_ref().unwrap();
        }

        let executor: &mut Executor = instance_data.event_executor.borrow_mut();
        let event_id = executor.next_event_id().unwrap();

        executor.spawn_with_event_id((*instance_data.memory_writer).clone(), __aws_dynamodb_list_tables_impl(), event_id);

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
            name_map.entry(list_tables_name).or_insert(aws_dynamodb_list_tables_impl as fn(&mut vm::Ctx) -> i32);

            let mut namespace_map = HashMap::new();
            namespace_map.entry(namespace).or_insert(name_map);

            registry.modules.entry(org).or_insert(namespace_map);
        }
    }

}
