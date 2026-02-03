use crate::{adapter::PhecdaAdapter, client::options::ClientOptions};

pub mod options;

pub struct PhecdaClient {
    options: ClientOptions,
    adapter: PhecdaAdapter,
}

impl PhecdaClient {
    pub fn from_options(options: ClientOptions) -> Self {
        Self {
            options,
            adapter: PhecdaAdapter::new(),
        }
    }
}
