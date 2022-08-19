use std::rc::Rc;
use std::sync::Arc;

use crate::providers::{Options, Provider, ProviderError};
use crate::transpiler::{Artifact, Bindable, Bootable, Castable, CastError};
use crate::transpiler::context::Context;

pub struct DnsProvider {
    options: Arc<Options>,
}

impl DnsProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Castable for DnsProvider {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        todo!()
    }
}

impl Bindable for DnsProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        todo!()
    }
}

impl Bootable for DnsProvider {
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        todo!()
    }

    fn is_booted(&self, ctx: Rc<Context>) -> bool {
        todo!()
    }
}

impl Provider for DnsProvider {
    fn name(&self) -> String {
        todo!()
    }

    fn options(&self) -> Arc<Options> {
        todo!()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        todo!()
    }
}