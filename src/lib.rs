#[doc(hidden)]
pub mod ex {
    pub use http::{request, Method};
    pub use std::prelude;
}

/// Create a GET request
#[macro_export]
macro_rules! get {
    ($uri:expr) => {
        get!($uri,)
    };
    ($uri:expr, $($k:tt: $v:tt),*) => {{
        use $crate::ex::request;
        use $crate::ex::prelude::*;

        let request = request::Builder::new()
            .method(Method::GET)
            .uri($uri)
            $(.header($k, $v))*
            .body(Default::default());

        match request {
            Ok(v) => v,
            Err(e) => panic!("invalid request; err = {:?}", e),
        }
    }};
}

/// Create a POST request
#[macro_export]
macro_rules! post {
    ($uri:expr, $body:expr) => {
        post!($uri, $body,)
    };
    ($uri:expr, $body:expr, $($k:tt: $v:tt),*) => {{
        use $crate::ex::request;
        use $crate::ex::prelude::*;

        let request = request::Builder::new()
            .method(Method::POST)
            .uri($uri)
            $(.header($k, $v))*
            .body($body);

        match request {
            Ok(v) => v,
            Err(e) => panic!("invalid request; err = {:?}", e),
        }
    }};
}
