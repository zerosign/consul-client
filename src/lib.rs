// GET http://127.0.0.1:8500/v1/agent/members
//
//
extern crate env_logger;
extern crate hyper;
extern crate log;
extern crate serde;
extern crate tokio;

use serde::Deserialize;
use std::{collections::BTreeMap, fmt, time::Instant};

#[cfg(test)]
mod tests {
    use super::Member;
    use crate::env_logger;
    use crate::hyper::{
        rt::{self, Future, Stream},
        Client, Uri,
    };
    use crate::log::debug;
    use crate::serde::Deserialize;
    use crate::tokio::runtime::Runtime;

    #[test]
    fn it_works() {
        env_logger::init();

        let client = Client::new();

        let task = client
            .get(Uri::from_static("http://127.0.0.1:8500/v1/agent/members"))
            .and_then(|res| res.into_body().concat2())
            .from_err::<FetchError>()
            .and_then(|body| {
                let members: Vec<Member> = serde_json::from_slice(&body)?;
                Ok(members)
            });

        let mut runtime = Runtime::new().expect("create this thread runtime");

        let result = runtime.block_on(task);

        debug!("{:?}", result);

        runtime.shutdown_on_idle();
    }
}
