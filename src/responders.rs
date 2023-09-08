use anyhow::Result;
use http::StatusCode;
use spin_sdk::http::Response;

pub fn to_response<T>(status: StatusCode, result: Option<T>) -> Result<Response>
where
    T: serde::Serialize,
{
    let mut builder = http::Response::builder();
    let mut body = None;

    if let Some(result) = result {
        let response = serde_json::to_string_pretty(&result)?.as_bytes().to_vec();

        builder = builder.header("Content-Type", "application/json");

        body = Some(response)
    }

    Ok(builder.status(status).body(body.map(|body| body.into()))?)
}
