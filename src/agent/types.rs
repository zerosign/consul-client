use futures::{Future, Stream};
use crate::agent::data::{Member, MemberConfig, MemberCoord, LocalMember};

//
// [Agent Query API](https://www.consul.io/api/agent.html)
//
pub trait AgentQuery {
    //
    // https://www.consul.io/api/agent.html#list-members
    //
    // GET http://127.0.0.1:8500/v1/agent/members
    fn members(&self) -> Future<Item = Vec<Member>, Error = FetchError>;

    //
    // https://www.consul.io/api/agent.html#read-configuration
    //
    // GET http://127.0.0.1:8500/v1/agent/self
    fn local(&self) -> Future<Item = LocalMember, Error = FetchError>;

    //
    // https://www.consul.io/api/agent.html#reload-agent
    //
    // PUT http://127.0.0.1:8500/v1/agent/reload
    fn reload(&self) -> Future<Item = (), Error = FetchError>;

    //
    // https://www.consul.io/api/agent.html#enable-maintenance-mode
    //
    // PUT http://127.0.0.1:8500/v1/agent/maintenance?enable=true&reason=For+API+docs
    fn maintain(&self, flag: bool, reason: Option<String>>) -> Future<Item = (), Error = FetchError>;

    //
    // https://www.consul.io/api/agent.html#view-metrics
    //
    // GET http://127.0.0.1:8500/v1/agent/metrics
    fn metrics(&self, format: Option<AgentMetric>>) -> Future<Item = Vec<u8>, Error = FetchError>;

    //
    // https://www.consul.io/api/agent.html#join-agent
    //
    // PUT http://127.0.0.1:8500/v1/agent/join/1.2.3.4
    fn join(&self, addr: String, wan: Option<bool>>) -> Future<Item = (), Error = FetchError>;

    //
    //
    // https://www.consul.io/api/agent.html#graceful-leave-and-shutdown
    // PUT http://127.0.0.1:8500/v1/agent/leave
    //
    // https://www.consul.io/api/agent.html#force-leave-and-shutdown
    // PUT http://127.0.0.1:8500/v1/agent/force-leave/agent-one
    //
    fn leave(&self, node: Option<String>) -> Future<Item = (), Error = FetchError>;

    //
    // https://www.consul.io/api/agent.html#stream-logs
    //
    // GET http://127.0.0.1:8500/v1/agent/monitor
    fn logs(&self, level: Option<Level>) -> Stream<Item = Message, Error = FetchError>;
}
