#[macro_use]
extern crate assemblylift_core_iomod_guest;

export_iomod_guest!(akkoro, aws, dynamodb);

use serde_json;

use assemblylift_core_io_guest::Io;

use crate::structs::{
    DeleteItemInput, DeleteItemOutput, GetItemInput, GetItemOutput, ListTablesInput,
    ListTablesOutput, PutItemInput, PutItemOutput, UpdateItemInput, UpdateItemOutput,
};

mod serialization;
pub mod structs;

call!(list_tables, ListTablesInput => ListTablesOutput);
call!(put_item, PutItemInput => PutItemOutput);
call!(get_item, GetItemInput => GetItemOutput);
call!(delete_item, DeleteItemInput => DeleteItemOutput);
call!(update_item, UpdateItemInput => UpdateItemOutput);

#[macro_export]
macro_rules! val {
    (B => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.b = Some($val);
        attr
    }};
    (S => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.s = Some($val.to_string());
        attr
    }};
    (N => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.n = Some($val.to_string());
        attr
    }};
}
