use crate::{error::ConsulError, futures::Future};

trait Status {
    //
    // https://www.consul.io/api/status.html#get-raft-leader
    //
    // GET /status/leader
    //
    fn status(&self) -> Future<Item = String, Error = ConsulError>;

    //
    //
    // https://www.consul.io/api/status.html#list-raft-peers
    //
    // GET /status/peers
    //
    fn peers(&self) -> Future<Item = Vec<String>, Error = ConsulError>;
}
