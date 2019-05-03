use http::Method;
use http_macros::post;

#[test]
fn only_uri_body() {
    let request = post!("/hello", "world");

    assert_eq!(request.method(), Method::POST);
    assert_eq!(*request.body(), "world");
    assert!(request.headers().is_empty());
}

#[test]
fn uri_body_headers() {
    let request = post!("/hello", "world", "foo": "bar");

    assert_eq!(request.method(), Method::POST);
    assert_eq!(*request.body(), "world");
    assert_eq!(request.headers().len(), 1);
    assert_eq!(request.headers()["foo"], "bar");
}
