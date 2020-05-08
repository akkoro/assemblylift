use assemblylift_core_event_guest::*;

extern {
    fn __asml_abi_invoke(ptr: *const u8, len: usize) -> i32;
}

pub mod database {
    use serde::{Deserialize, Serialize};
    use assemblylift_core_event_guest::Event;

    /// <p>Represents the output of a <code>ListTables</code> operation.</p>
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTablesOutput {
        /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p> <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
        #[serde(rename = "LastEvaluatedTableName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_evaluated_table_name: Option<String>,
        /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p> <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
        #[serde(rename = "TableNames")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub table_names: Option<Vec<String>>,
    }

    pub fn aws_dynamodb_list_tables<'a>() -> Event<'a, ListTablesOutput> {
        let event_id: i32;
        unsafe {
            let name = "aws.dynamodb.list_tables";
            event_id = crate::__asml_abi_invoke(name.as_ptr(), name.len())
        }

        match event_id {
            -1 => panic!("unable to invoke fn"),
            _ => Event::<ListTablesOutput>::new(event_id as u32)
        }
    }
}
