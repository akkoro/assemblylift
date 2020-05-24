#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate assemblylift_core;

pub mod database {
    use std::collections::HashMap;
    use std::future::Future;

    use crossbeam_utils::atomic::AtomicCell;
    use rusoto_core::Region;
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
    use serde_json;
    use wasmer_runtime_core::vm;

    use assemblylift_core::iomod::{AsmlAbiFn, IoModule, ModuleRegistry};
    use assemblylift_core::WasmBufferPtr;
    use assemblylift_core_event::threader::Threader;
    use assemblylift_core_event_common::constants::EVENT_BUFFER_SIZE_BYTES;

    lazy_static! {
        static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    }

    // aws.dynamodb.list_tables

    async fn __aws_dynamodb_list_tables_impl() -> Vec<u8> {
        println!("TRACE: __aws_dynamodb_list_tables_impl");

        let result = DYNAMODB.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None,
        }).await.unwrap();

        serde_json::to_vec(&result).unwrap()
    }

    pub fn aws_dynamodb_list_tables_impl(ctx: &mut vm::Ctx, mem: WasmBufferPtr, _input: WasmBufferPtr, _input_len: u32) -> i32 {
        println!("TRACE: aws_dynamodb_list_tables_impl");
        spawn_event(ctx, mem, __aws_dynamodb_list_tables_impl())
    }

    // aws.dynamodb.put_item

    async fn __aws_dynamodb_put_item_impl(input: Vec<u8>) -> Vec<u8> {
        println!("TRACE: __aws_dynamodb_put_item_impl");

        let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
        let result = DYNAMODB.put_item(deserialized).await.unwrap();
        serde_json::to_vec(&result).unwrap()
    }

    pub fn aws_dynamodb_put_item_impl(ctx: &mut vm::Ctx, mem: WasmBufferPtr, input: WasmBufferPtr, input_len: u32) -> i32 {
        println!("TRACE: aws_dynamodb_put_item_impl");

        let input_vec = wasm_buffer_as_vec(ctx, input, input_len);
        spawn_event(ctx, mem, __aws_dynamodb_put_item_impl(input_vec))
    }

    // aws.dynamodb.get_item

    async fn __aws_dynamodb_get_item_impl(input: Vec<u8>) -> Vec<u8> {
        println!("TRACE: __aws_dynamodb_get_item_impl");

        let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
        let result = DYNAMODB.get_item(deserialized).await.unwrap();
        serde_json::to_vec(&result).unwrap()
    }
    
    pub fn aws_dynamodb_get_item_impl(ctx: &mut vm::Ctx, mem: WasmBufferPtr, input: WasmBufferPtr, input_len: u32) -> i32 {
        println!("TRACE: aws_dynamodb_get_item_impl");

        let input_vec = wasm_buffer_as_vec(ctx, input, input_len);
        spawn_event(ctx, mem, __aws_dynamodb_get_item_impl(input_vec))
    }

    // aws.dynamodb.delete_item

    async fn __aws_dynamodb_delete_item_impl(input: Vec<u8>) -> Vec<u8> {
        println!("TRACE: __aws_dynamodb_delete_item_impl");

        let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
        let result = DYNAMODB.delete_item(deserialized).await.unwrap();
        serde_json::to_vec(&result).unwrap()
    }

    pub fn aws_dynamodb_delete_item_impl(ctx: &mut vm::Ctx, mem: WasmBufferPtr, input: WasmBufferPtr, input_len: u32) -> i32 {
        println!("TRACE: aws_dynamodb_delete_item_impl");

        let input_vec = wasm_buffer_as_vec(ctx, input, input_len);
        spawn_event(ctx, mem, __aws_dynamodb_delete_item_impl(input_vec))
    }

    // aws.dynamodb.update_item

    async fn __aws_dynamodb_update_item_impl(input: Vec<u8>) -> Vec<u8> {
        println!("TRACE: __aws_dynamodb_update_item_impl");

        let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
        let result = DYNAMODB.update_item(deserialized).await.unwrap();
        serde_json::to_vec(&result).unwrap()
    }

    pub fn aws_dynamodb_update_item_impl(ctx: &mut vm::Ctx, mem: WasmBufferPtr, input: WasmBufferPtr, input_len: u32) -> i32 {
        println!("TRACE: aws_dynamodb_update_item_impl");

        let input_vec = wasm_buffer_as_vec(ctx, input, input_len);
        spawn_event(ctx, mem, __aws_dynamodb_update_item_impl(input_vec))
    }

    // helpers

    fn wasm_buffer_as_vec(ctx: &mut vm::Ctx, input: WasmBufferPtr, input_len: u32) -> Vec<u8> {
        let wasm_instance_memory = ctx.memory(0);
        let input_deref: &[AtomicCell<u8>] = input
            .deref(wasm_instance_memory, 0, input_len)
            .unwrap();

        let mut as_vec: Vec<u8> = Vec::new();
        for (idx, b) in input_deref.iter().enumerate() {
            as_vec.insert(idx, b.load());
        }

        as_vec
    }

    fn spawn_event(ctx: &mut vm::Ctx, mem: WasmBufferPtr, future: impl Future<Output=Vec<u8>> + 'static + Send) -> i32 {
        let threader: *mut Threader = ctx.data.cast();
        let threader_ref = unsafe { threader.as_mut().unwrap() };

        let event_id = threader_ref.next_event_id().unwrap();
        println!("DEBUG: event_id={}", event_id);

        let wasm_instance_memory = ctx.memory(0);
        let memory_writer: &[AtomicCell<u8>] = mem
            .deref(wasm_instance_memory, 0, EVENT_BUFFER_SIZE_BYTES as u32)
            .unwrap();

        threader_ref.spawn_with_event_id(memory_writer.as_ptr(), future, event_id);

        event_id as i32
    }

    // iomod interface

    pub struct MyModule {}

    impl IoModule for MyModule {
        fn register(registry: &mut ModuleRegistry) {
            register_calls!(registry, 
                aws => {
                    dynamodb => {
                        list_tables => aws_dynamodb_list_tables_impl,
                        put_item => aws_dynamodb_put_item_impl,
                        get_item => aws_dynamodb_get_item_impl,
                        delete_item => aws_dynamodb_delete_item_impl,
                        update_item => aws_dynamodb_update_item_impl
                    }
                }
            );
        }
    }

}
