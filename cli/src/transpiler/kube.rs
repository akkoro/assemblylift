pub mod root {
    use std::rc::Rc;

    use crate::transpiler::{asml, Castable, CastError, ContentType};

    pub struct Block;

    impl Block {
        pub fn new() -> Self {
            Block {}
        }
    }

    impl Castable for Block {
        fn cast(&mut self, ctx: Rc<asml::Context>, name: &str) -> Result<Vec<String>, CastError> {
            todo!()
        }

        fn content_type(&self) -> Vec<ContentType> {
            vec![ContentType::KubeYaml("kube-yaml")]
        }
    }
}
