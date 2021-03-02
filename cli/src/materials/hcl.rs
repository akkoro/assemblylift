pub mod service {
    use std::convert::From;

    use crate::providers::*;
    use crate::materials::{hcl, Artifact, ContentType, Source};

    pub type ArtifactList = Vec<Box<hcl::function::Module>>;

    pub struct Module {
        hcl: String,
        functions: ArtifactList,
    }

    impl Module {
        pub fn new(content: String, functions: ArtifactList) -> Self {
            Module { hcl: content, functions }
        }
    }

    impl Artifact for Module {
        fn content_type(&self) -> ContentType {
            ContentType::HCL("HCL")
        }
        
        fn content(&self) -> String {
            self.hcl
        }
    }

    impl<T> From<T> for Module 
    where
        T: Source
    {
        fn from(root: T) -> Self {
            let service = root.service();
            let functions = root.functions();

            let provider_name = service.provider.unwrap_or(String::from("aws-lambda"));
            let service_provider = SERVICE_PROVIDERS.get(&provider_name).expect("could not find provider by name");

            let mut content = String::from("");

            let service_artifact = service_provider.transform(service).unwrap();
            content.push_str(&service_artifact.content());

            let function_modules: ArtifactList = ArtifactList::new(); 
            for (_id, function) in functions {
                let module = hcl::function::Module::from(function);
                function_modules.push(Box::new(module));
            }
            
            Self::new(content, function_modules)
        }
    }
}

pub mod function {
    use std::convert::From;

    use crate::providers::*;
    use crate::materials::{models, Artifact, ContentType};

    pub struct Module {
        hcl: String,
    }

    impl Module {
        fn new(content: String) -> Self {
            Self { hcl: content }
        }
    }
    
    impl Artifact for Module {
        fn content_type(&self) -> ContentType {
            ContentType::HCL("HCL")
        }
        
        fn content(&self) -> String {
            self.hcl
        }
    }

    impl From<models::Function> for Module {
        fn from(function: models::Function) -> Self {
            let provider_name = function.provider.unwrap_or(String::from("aws-lambda"));
            let function_provider = FUNCTION_PROVIDERS.get(&provider_name).expect("could not find provider by name");
            
            let function_artifact = function_provider.transform(function).unwrap();
            Self::new(function_artifact.content())
        }
    }
}
