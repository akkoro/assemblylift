#[macro_use]
extern crate lazy_static;

pub mod database {
    use rusoto_dynamodb::{DynamoDbClient, DynamoDb, ListTablesInput, ListTablesOutput, ListTablesError};
    use rusoto_core::Region;

    use wasmer_runtime::{Instance, Ctx, Func};
    use wasmer_runtime_core::vm;

    use assemblylift_core_event::*;
    use assemblylift_core::{serialize_event_from_host, InstanceData, serialize_future_from_host};

    lazy_static! {
        static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    }

    pub fn aws_dynamodb_list_tables_impl(ctx: &mut vm::Ctx) -> i32 {
        println!("TRACE: Called aws_dynamodb_list_tables_impl");

        // let ddb = DynamoDbClient::new(Region::UsEast1);
        let mut rusoto_future = DYNAMODB.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None
        });

        let mut instance_data: &mut InstanceData;
        unsafe {
            instance_data = *ctx.data.cast::<&mut InstanceData>();
            serialize_future_from_host(0, Box::new(rusoto_future), ctx);
        }

        // Write the event into the event buffer, accessible by WASM
        // let event_index = event.inner.id;
        // unsafe {
            // MUSTDO catch errors from unsafe code
            // serialize_event_from_host(event_index, &event, ctx);
        // }

        // event_index as i32
        0
    }

    use std::ops::DerefMut;
    use std::collections::HashMap;
    use assemblylift_core::iomod::{ModuleRegistry, IoModule};
    use std::sync::Mutex;
    use assemblylift_core_event::manager::EventManager;
    use futures::task::{Context, Poll};
    use std::pin::Pin;
    use std::ffi::c_void;

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
