#[cfg(feature = "host")]
pub mod database {
    use rusoto_dynamodb::{DynamoDbClient, DynamoDb, ListTablesInput};
    use rusoto_core::{Region, RusotoFuture};
    use std::future::Future;

    use wasmer_runtime::Instance;

    use assemblylift_core::event::*;
    use assemblylift_core::event::serde::serialize_event_from_host;

    // MOVE RusotoEvent to a separate module
    struct RusotoEvent(Event);

    impl<O, E> From<RusotoFuture<O, E>> for RusotoEvent {
        fn from(f: RusotoFuture<O, E>) -> Self {
            // TODO the rusoto future needs to be bound to the event id somehow
            //      may need to stub out the event manager - this can bind events to futures in a hashmap
            unimplemented!()
        }
    }

    /// called from host:main.rs, where it is bound to __wsw_list_tables
    pub fn aws_dynamodb_list_tables_impl(instance: Box<Instance>) -> i32 {
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
            serialize_event_from_host(event_index, &e.0, instance.as_ref());
        }

        event_index as i32
    }

}

#[cfg(feature = "client")]
pub mod client;
