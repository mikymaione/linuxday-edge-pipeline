use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

#[http_component]
async fn handle_edge_gateway(req: Request) -> anyhow::Result<impl IntoResponse> {
    match *req.method() {
        Method::Post => {
            let body_bytes = req.body().to_vec();

            // Inoltra a /enrich
            let enrich_req = Request::post("http://127.0.0.1:3000/enrich", body_bytes);
            let enrich_res: Response = spin_sdk::http::send(enrich_req).await?;
            let enrich_body = enrich_res.body().to_vec();

            // Inoltra l'esito a /audit specificando esplicitamente il tipo Response
            let audit_req = Request::post("http://127.0.0.1:3000/audit", enrich_body.clone());
            let _audit_res: Response = spin_sdk::http::send(audit_req).await?;

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(enrich_body)
                .build())
        }
        _ => Ok(Response::builder().status(405).body("Method Not Allowed").build()),
    }
}