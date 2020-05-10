use assemblylift_core_event_guest::*;

extern {
    fn __asml_abi_invoke(mem: *const u8, ptr: *const u8, len: usize) -> i32;
}

pub mod database {
    use serde::{Deserialize, Serialize};
    use assemblylift_core_event_guest::{Event, EVENT_BUFFER};

    // FIXME these structs are copied from rusoto -- we can't pull it in as a dependency
    //   since we get a bunch of wasm-incompatible stuff with the generated structs

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTablesOutput {
        #[serde(rename = "LastEvaluatedTableName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_evaluated_table_name: Option<String>,

        #[serde(rename = "TableNames")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub table_names: Option<Vec<String>>,
    }

    pub fn aws_dynamodb_list_tables<'a>() -> Event<'a, ListTablesOutput> {
        let event_id: i32;
        unsafe {
            let name = "aws.dynamodb.list_tables";
            event_id = crate::__asml_abi_invoke(EVENT_BUFFER.as_ptr(), name.as_ptr(), name.len());
        }

        match event_id {
            -1 => panic!("unable to invoke fn"),
            _ => Event::<ListTablesOutput>::new(event_id as u32)
        }
    }
}
