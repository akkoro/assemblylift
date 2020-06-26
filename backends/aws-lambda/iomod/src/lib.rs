#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate assemblylift_core;
extern crate paste;

pub mod database {
    use std::collections::HashMap;

    use crossbeam_utils::atomic::AtomicCell;
    use paste;
    use rusoto_core::Region;
    use rusoto_dynamodb::DynamoDbClient;
    use serde_json;
    use wasmer_runtime_core::vm;

    use assemblylift_core::iomod::registry::{AsmlAbiFn, ModuleRegistry};
    use assemblylift_core::iomod::IoModule;
    use assemblylift_core::WasmBufferPtr;

    lazy_static! {
        static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    }

    // aws.dynamodb.list_tables
    call!(aws_dynamodb_list_tables =>
        pub async fn aws_dynamodb_list_tables_impl(input: Vec<u8>) -> Vec<u8> {
            use rusoto_dynamodb::*;

            let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
            let result = DYNAMODB.list_tables(deserialized).await.unwrap();
            serde_json::to_vec(&result).unwrap()
        }
    );

    // aws.dynamodb.put_item
    call!(aws_dynamodb_put_item =>
        pub async fn aws_dynamodb_put_item_impl(input: Vec<u8>) -> Vec<u8> {
            use rusoto_dynamodb::*;

            let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
            let result = DYNAMODB.put_item(deserialized).await.unwrap();
            serde_json::to_vec(&result).unwrap()
        }
    );

    // aws.dynamodb.get_item
    call!(aws_dynamodb_get_item =>
        pub async fn aws_dynamodb_get_item_impl(input: Vec<u8>) -> Vec<u8> {
            use rusoto_dynamodb::*;

            let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
            let result = DYNAMODB.get_item(deserialized).await.unwrap();
            serde_json::to_vec(&result).unwrap()
        }
    );

    // aws.dynamodb.delete_item
    call!(aws_dynamodb_delete_item =>
        pub async fn aws_dynamodb_delete_item_impl(input: Vec<u8>) -> Vec<u8> {
            use rusoto_dynamodb::*;

            let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
            let result = DYNAMODB.delete_item(deserialized).await.unwrap();
            serde_json::to_vec(&result).unwrap()
        }
    );

    // aws.dynamodb.update_item
    call!(aws_dynamodb_update_item =>
        pub async fn aws_dynamodb_update_item_impl(input: Vec<u8>) -> Vec<u8> {
            use rusoto_dynamodb::*;

            let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
            let result = DYNAMODB.update_item(deserialized).await.unwrap();
            serde_json::to_vec(&result).unwrap()
        }
    );

    // iomod interface

    pub struct MyModule {}

    impl IoModule for MyModule {
        fn register(registry: &mut ModuleRegistry) {
            register_calls!(registry,
                aws => {
                    dynamodb => {
                        list_tables => aws_dynamodb_list_tables,
                        put_item => aws_dynamodb_put_item,
                        get_item => aws_dynamodb_get_item,
                        delete_item => aws_dynamodb_delete_item,
                        update_item => aws_dynamodb_update_item
                    }
                }
            );
        }
    }
}
