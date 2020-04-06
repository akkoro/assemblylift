extern {
    fn __asml_abi_invoke(ptr: *const u8, len: usize) -> i32;
}

pub mod database {
    pub fn aws_dynamodb_list_tables() -> i32 {
        unsafe {
            let name = "aws.dynamodb.list_tables";
            crate::__asml_abi_invoke(name.as_ptr(), name.len())
            // TODO: where does get_response come from?
        }
    }
}
