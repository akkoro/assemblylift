use crate::ContentType;

pub trait Driver {
    fn content_type(&self) -> ContentType;
    fn extension(&self) -> String;
    fn name(&self) -> String;
}
