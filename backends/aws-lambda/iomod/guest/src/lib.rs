pub mod structs;
mod serialization;

extern {
    fn __asml_abi_invoke(mem: *const u8, name_ptr: *const u8, name_len: usize, input_ptr: *const u8, input_len: usize) -> i32;
}

pub mod database {
    use serde_json;
    use paste;

    use assemblylift_core_guest::*;
    use assemblylift_core_event_guest::Event;

    use crate::structs::{ListTablesInput, ListTablesOutput, PutItemInput, PutItemOutput, 
        GetItemInput, GetItemOutput, DeleteItemInput, DeleteItemOutput, UpdateItemInput, UpdateItemOutput};

    call!(aws => dynamodb => list_tables, ListTablesInput => ListTablesOutput);
    call!(aws => dynamodb => put_item, PutItemInput => PutItemOutput);
    call!(aws => dynamodb => get_item, GetItemInput => GetItemOutput);
    call!(aws => dynamodb => delete_item, DeleteItemInput => DeleteItemOutput);
    call!(aws => dynamodb => update_item, UpdateItemInput => UpdateItemOutput);

    #[macro_export]
    macro_rules! val {
        (B => $val:expr) => (
            {
                let mut attr = AttributeValue::default();
                attr.b = Some($val);
                attr
            }
        );
        (S => $val:expr) => (
            {
                let mut attr = AttributeValue::default();
                attr.s = Some($val.to_string());
                attr
            }
        );
        (N => $val:expr) => (
            {
                let mut attr = AttributeValue::default();
                attr.n = Some($val.to_string());
                attr
            }
        );
    }
}
