#[macro_use]
extern crate lazy_static;

pub mod database {
    use rusoto_dynamodb::{DynamoDbClient, DynamoDb, ListTablesInput, ListTablesOutput, ListTablesError};
    use rusoto_core::Region;

    use wasmer_runtime::{Instance, Ctx, Func};
    use wasmer_runtime_core::vm;

    use assemblylift_core_event::*;
    use assemblylift_core::InstanceData;

    lazy_static! {
        static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    }

    async fn __aws_dynamodb_list_tables_impl(event_id: u32) -> Vec<u8> {
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
        unsafe {
            instance_data = *ctx.data.cast::<&mut InstanceData>();
        }

        // TODO need to track async calls & write their results to a known location in shared mem.
        //      possible approach using async wrapper as started already:
        //      1. get an unused event index from the executor
        //      2. get future from __impl; pass the event index to this impl so it knows where to Wr
        //      3. spawn this wrapper future w/ instance_data.event_executor.spawner.spawn(func)
        //      4. return the event index; the id is returned to the guest & wrapped in Event

        let executor: &Executor = instance_data.event_executor.borrow();
        let event = executor.make_event().unwrap();
        let event_id = event.id.clone();
        let func = __aws_dynamodb_list_tables_impl(event_id);

        executor.spawn_as_event(func, &event);

        event_id as i32
    }

    use std::ops::DerefMut;
    use std::collections::HashMap;
    use assemblylift_core::iomod::{ModuleRegistry, IoModule};
    use std::sync::Mutex;
    use assemblylift_core_event::manager::{EventManager, DynFut};
    use std::pin::Pin;
    use std::ffi::c_void;
    use futures::TryFutureExt;
    use std::borrow::Borrow;
    use assemblylift_core_event::executor::Executor;

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
