use assemblylift_core_event::*;

extern {
    fn __asml_abi_invoke(ptr: *const u8, len: usize) -> i32;
}

pub mod database {
    use assemblylift_core_event::Event;

    pub fn aws_dynamodb_list_tables() -> Option<Event> {
        let event_id: i32;
        unsafe {
            let name = "aws.dynamodb.list_tables";
            event_id = crate::__asml_abi_invoke(name.as_ptr(), name.len())
        }

        match event_id {
            -1 => None,
            _ => Some(Event::new(event_id as u32))
        }
    }
}
