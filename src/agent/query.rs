use crate::agent::types::AgentQuery;
use crate::client;
use crate::error::ConsulError;
use crate::hyper::{Client as HyperClient, Uri};

impl AgentQuery for client::Client {
    fn members(&self) -> Future<Item = Vec<Member>, Error = ConsulError> {
        // somehow I prefer using macro in here rather than having a generic method fetch
    }
}
