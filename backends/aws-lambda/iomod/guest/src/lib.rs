use assemblylift_core_event_guest::*;

pub mod structs;
mod serialization;

extern {
    fn __asml_abi_invoke(mem: *const u8, name_ptr: *const u8, name_len: usize, input_ptr: *const u8, input_len: usize) -> i32;
}

pub mod database {
    use serde_json;

    use assemblylift_core_event_guest::{Event, EVENT_BUFFER};

    use crate::structs::{ListTablesOutput, PutItemInput, PutItemOutput};

    pub fn aws_dynamodb_list_tables<'a>() -> Event<'a, ListTablesOutput> {
        let event_id: i32;
        unsafe {
            let input_ptr: *const u8 = 0 as *const u8;
            let name = "aws.dynamodb.list_tables";
            event_id = crate::__asml_abi_invoke(EVENT_BUFFER.as_ptr(),
                                                name.as_ptr(), name.len(),
                                                input_ptr, 0);
        }

        match event_id {
            -1 => panic!("unable to invoke fn"),
            _ => Event::<ListTablesOutput>::new(event_id as u32)
        }
    }

    pub fn aws_dynamodb_put_item<'a>(input: PutItemInput) -> Event<'a, PutItemOutput> {
        let event_id: i32;
        unsafe {
            let serialized: Box<Vec<u8>> = Box::from(serde_json::to_vec(&input).unwrap());


            let name = "aws.dynamodb.put_item";
            event_id = crate::__asml_abi_invoke(EVENT_BUFFER.as_ptr(),
                                                name.as_ptr(), name.len(),
                                                serialized.as_ptr(), serialized.len());
        }

        match event_id {
            -1 => panic!("unable to invoke fn"),
            _ => Event::<PutItemOutput>::new(event_id as u32)
        }
    }
}
