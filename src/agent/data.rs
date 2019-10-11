#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Member {
    addr: String,
    port: u32,
    name: String,
    status: u32,
    tags: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MemberConfig {
    datacenter: String,
    node_name: String,
    node_id: String,
    server: bool,
    revision: String,
    version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MemberCoord {
    adjustment: u32,
    error: f32,
    r#vec: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalMember {
    config: MemberConfig,
    coord: MemberCoord,
    member: Member,
    status: u32,
    meta: BTreeMap<String, String>,
}

pub enum MetricFormat {
    Prometheus,
    Json,
}

#[derive(Debug)]
pub enum Severity {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug)]
pub struct Message {
    created_at: Instant,
    level: Level,
    group: String,
    message: String,
}
