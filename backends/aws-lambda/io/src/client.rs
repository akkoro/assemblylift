pub mod database {
    extern {
        fn __wsw_list_tables();
    }

    pub fn aws_dynamodb_list_tables() {
        unsafe {
            __wsw_list_tables();
            // TODO: where does get_response come frome?
        }
    }
}