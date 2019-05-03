use http::{Method, Request};
use http_macros::get;

#[test]
fn only_uri() {
    let request: Request<()> = get!("/hello");

    assert_eq!(request.method(), Method::GET);
    assert!(request.headers().is_empty());
}

#[test]
fn uri_body_headers() {
    let request: Request<()> = get!("/hello", "foo": "bar");

    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.headers().len(), 1);
    assert_eq!(request.headers()["foo"], "bar");
}
