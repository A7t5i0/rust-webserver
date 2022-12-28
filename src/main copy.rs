use http::{Request, Response};

fn response(req: Request<()>) -> http::Result<Response<()>> {
    match req.uri().path() {
        "/" => index(req),
        "/foo" => foo(req),
        "/bar" => bar(req),
        _ => not_found(req),
    }
}

use http::{HeaderValue, Response, StatusCode};
use http::header::CONTENT_TYPE;

fn add_server_headers<T>(response: &mut Response<T>) {
    response.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *response.status_mut() = StatusCode::OK;

    use http::header::{self, HeaderName};

    let name: HeaderName = header::ACCEPT;
    assert_eq!(name.as_str(), "accept");

    use http::HeaderValue;

    let value = "text/html";
    let value = value.parse::<HeaderValue>().unwrap();
    use http::Uri;
    use http::uri::Scheme;

    let uri = "http://127.0.0.1:7878/index.html".parse::<Uri>().unwrap();

    assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
    assert_eq!(uri.host(), Some("www.rust-lang.org"));
    assert_eq!(uri.path(), "/index.html");
    assert_eq!(uri.query(), None);
}



fn main() {
    println!("Hello, world!");
}
