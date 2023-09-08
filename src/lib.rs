use anyhow::{Ok, Result};
use http::Method;
use responders::to_response;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod extractors;
mod fibo;
mod responders;

#[http_component]
fn handle_fibonacci(req: Request) -> Result<Response> {
    let path = req.uri().path().to_string();
    if path.starts_with("/fibonacci/") {
        match *req.method() {
            Method::GET => {
                let base_number = extractors::extract_base_number(&req);
                print!("{}", base_number);
                let result = fibo::fibonacci(base_number);
                match result {
                    Some(_) => to_response(http::StatusCode::OK, Some(result)),
                    None => to_response(http::StatusCode::BAD_REQUEST, Some("result is too big")),
                }
            }
            _ => Ok(http::Response::builder()
                .status(http::StatusCode::METHOD_NOT_ALLOWED)
                .body(None)?),
        }
    } else {
        Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?)
    }
}
