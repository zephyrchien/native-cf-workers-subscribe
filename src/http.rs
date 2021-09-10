use worker::Response;

#[inline]
pub fn forbidden() -> Response { Response::error("Forbidden", 403).unwrap() }

#[inline]
pub fn not_found() -> Response { Response::error("Not Found", 404).unwrap() }

#[inline]
pub fn new_response(message: &str) -> Response {
    Response::ok(message).unwrap()
}
