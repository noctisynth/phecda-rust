use crate::client::{PhecdaClient, options::ClientOptions};
use std::sync::Arc;
pub struct ClientInstance(Arc<PhecdaClient>);

pub fn init<T>(opt: T) -> ClientInstance
where
    T: Into<ClientOptions>,
{
    ClientInstance(Arc::new(PhecdaClient::from_options(opt.into())))
}
