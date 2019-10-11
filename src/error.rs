use hyper::Error as HyperError;
use serde_json::Error as SerdeError;

// Define a type so we can return multiple types of errors
#[derive(Debug)]
enum ConsulError {
    Http(HyperError),
    Json(SerdeError),
}

impl From<HyperError> for FetchError {
    #[inline]
    fn from(err: HyperError) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<SerdeError> for FetchError {
    #[inline]
    fn from(err: SerdeError) -> FetchError {
        FetchError::Json(err)
    }
}
