use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

#[http_component]
fn handle_audit_logger(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Apri il database "default" fornito dal runtime di Spin
    let conn = Connection::open_default()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            payload TEXT NOT NULL
        )",
        &[],
    )?;

    let payload_str = String::from_utf8_lossy(req.body()).to_string();
    conn.execute(
        "INSERT INTO audit_logs (payload) VALUES (?)",
        &[Value::Text(payload_str)],
    )?;

    Ok(Response::builder()
        .status(201)
        .header("content-type", "text/plain")
        .body("Logged")
        .build())
}