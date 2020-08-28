#[macro_use]
extern crate assemblylift_core_iomod_guest;

export_iomod_guest!();

use paste;
use serde_json;

use assemblylift_core_event_guest::Event;

use crate::structs::{
    DeleteItemInput, DeleteItemOutput, GetItemInput, GetItemOutput, ListTablesInput,
    ListTablesOutput, PutItemInput, PutItemOutput, UpdateItemInput, UpdateItemOutput,
};

mod serialization;
pub mod structs;

call!(aws => dynamodb => list_tables, ListTablesInput => ListTablesOutput);
call!(aws => dynamodb => put_item, PutItemInput => PutItemOutput);
call!(aws => dynamodb => get_item, GetItemInput => GetItemOutput);
call!(aws => dynamodb => delete_item, DeleteItemInput => DeleteItemOutput);
call!(aws => dynamodb => update_item, UpdateItemInput => UpdateItemOutput);

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
