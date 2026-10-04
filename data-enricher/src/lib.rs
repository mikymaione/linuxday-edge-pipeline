use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Payload {
    message: String,
    #[serde(default)]
    processed_by: Option<String>,
}

#[http_component]
fn handle_data_enricher(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut payload: Payload = serde_json::from_slice(req.body()).unwrap_or(Payload {
        message: "raw data".into(),
        processed_by: None,
    });

    payload.processed_by = Some("data-enricher-edge".into());
    let response_bytes = serde_json::to_vec(&payload)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response_bytes)
        .build())
}