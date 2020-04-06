pub mod database {
    use rusoto_dynamodb::{DynamoDbClient, DynamoDb, ListTablesInput};
    use rusoto_core::{Region, RusotoFuture};
    use std::future::Future;

    use wasmer_runtime::{Instance, Ctx, Func};

    use assemblylift_core_event::*;
    use assemblylift_core::serialize_event_from_host;

    // MOVE RusotoEvent to a separate module
    struct RusotoEvent(Event);

    impl<O, E> From<RusotoFuture<O, E>> for RusotoEvent {
        fn from(f: RusotoFuture<O, E>) -> Self {
            // TODO the rusoto future needs to be bound to the event id somehow
            //      may need to stub out the event manager - this can bind events to futures in a hashmap
            unimplemented!()
        }
    }

    pub fn aws_dynamodb_list_tables_impl() -> i32 {
        println!("TRACE: Called aws_dynamodb_list_tables_impl");

        let ddb = DynamoDbClient::new(Region::UsEast1);
        let rusoto_future = ddb.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None
        });

        let e = RusotoEvent::from(rusoto_future);

        // Write the event into the event buffer, accessible by WASM
        let event_index = e.0.state.id;
        unsafe {
            // MUSTDO catch errors from unsafe code
            // serialize_event_from_host(event_index, &e.0, ctx);
        }

        event_index as i32
    }

    use std::ops::DerefMut;
    use std::collections::HashMap;
    use assemblylift_core::iomod::{ModuleRegistry, IoModule, MODULE_REGISTRY};

    // requires compiling as cdylib
    // extern "C" {
    //     fn get_module_registry() -> Mutex<ModuleRegistry>;
    // }

    pub struct Module {}

    impl IoModule for Module {
        fn register() {
            // TODO a lot of this can be hidden by a macro

            let org = "aws".to_string();
            let namespace = "dynamodb".to_string();
            let list_tables_name = "list_tables".to_string();

            // TODO if this module were in a library, MODULE_REGISTRY would have to be extern
            let mut reg = MODULE_REGISTRY.lock().unwrap();
            // let mut reg = unsafe { get_module_registry() };

            let mut name_map = HashMap::new();
            name_map.entry(list_tables_name).or_insert(aws_dynamodb_list_tables_impl as fn() ->i32);

            let mut namespace_map = HashMap::new();
            namespace_map.entry(namespace).or_insert(name_map);

            reg.deref_mut().modules.entry(org).or_insert(namespace_map);
        }
    }

}
