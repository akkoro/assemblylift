#[cfg(feature = "host")]
pub mod database {
    use rusoto_dynamodb::{DynamoDbClient, DynamoDb, ListTablesInput};
    use rusoto_core::Region;

    pub fn aws_dynamodb_list_tables_impl() {
        let ddb = DynamoDbClient::new(Region::UsEast1);
        ddb.list_tables(ListTablesInput {
            exclusive_start_table_name: None,
            limit: None
        }).sync();

        // TODO
    }
}

#[cfg(feature = "client")]
pub mod client;
