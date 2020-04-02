#[cfg(feature = "host")]
pub mod database {
    use rusoto_dynamodb::{DynamoDbClient, DynamoDb, ListTablesInput};
    use rusoto_core::Region;
    use std::future::Future;

    extern crate core;
    use assemblylift_core::event::*;
    use assemblylift_core::event::serde::serialize_event;

    // ARCH called from host:main.rs, where it is bound to __wsw_list_tables
    pub fn aws_dynamodb_list_tables_impl() -> i32 {
        let ddb = DynamoDbClient::new(Region::UsEast1);
        ddb.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None
        });

        let e = &Event::new();

        // Write the event into the event buffer, accessible by WASM
        let event_index = e.state.id;
        unsafe {
            serialize_event(event_index, e);
        }

        event_index as i32
    }

}

#[cfg(feature = "client")]
pub mod client;
