use crate::hyper::{self, Uri};

pub struct Client {
    inner: hyper::Client,
}
