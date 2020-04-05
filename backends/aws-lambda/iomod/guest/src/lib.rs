pub mod database {
    extern {
        fn __wsw_list_tables() -> i32;
    }

    pub fn aws_dynamodb_list_tables() -> i32 {
        unsafe {
            __wsw_list_tables()
            // TODO: where does get_response come from?
        }
    }
}
