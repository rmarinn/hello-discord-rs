use worker::{event, Context, Env, Request, Response, ResponseBody, ResponseBuilder};

mod parse;
mod response;
mod verify;

pub use parse::*;
pub use response::*;
pub use verify::*;

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    let headers = match parse_headers(&req) {
        Ok(headers) => headers,
        Err(e) => return e,
    };

    let body = match parse_body(&mut req).await {
        Ok(body) => body,
        Err(e) => return e,
    };

    if let Err(e) = verify_signature(&headers, &body).await {
        return e;
    }

    let request_body: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(body) => body,
        Err(e) => return Response::error(format!("Failed to parse request: {}", e), 500),
    };

    if let Some(kind) = request_body.get("type") {
        if kind == 1 {
            let body = ResponseBody::Body(r#"{"type": 1}"#.into());
            let resp = ResponseBuilder::new()
                .with_header("content-type", "application/json")?
                .body(body);
            Ok(resp)
        } else if kind == 2 {
            let resp = InteractionResponse::Message(MessageBuilder::new().content("ok").build());
            Response::from_json(&resp)
        } else {
            Response::error("Interaction not implemented", 501)
        }
    } else {
        Response::error("Failed to parse request", 400)
    }
}
