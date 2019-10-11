use crate::{data::Metadata, error::ConsulError, futures::Future};

trait KV {
    fn read<S: Into<String>>(
        &self,
        key: S,
        dc: Option<S>,
        recurse: Option<bool>,
    ) -> Future<Item = Metadata, Error = ConsulError>;

    fn keys<S: Into<String>>(
        &self,
        key: S,
        dc: Option<S>,
    ) -> Future<Item = Vec<String>, Error = ConsulError>;

    fn put<S: Into<String>>(
        &self,
        key: S,
        dc: Option<S>,
        flags: u64,
    ) -> Future<Item = bool, Error = ConsulError>;

    fn delete<S: Into<String>>(
        &self,
        key: S,
        recurse: Option<bool>,
    ) -> Future<Item = bool, Error = ConsulError>;
}
