use serde_discord::{interaction::Interaction, response::InteractionResponse};
use worker::{event, Context, Env, Request, Response};

mod commands;
mod parse;
mod verify;

pub use commands::*;
pub use parse::*;
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

    let interaction: Interaction = serde_json::from_slice(&body)?;

    match interaction {
        Interaction::Ping => Response::from_json(&InteractionResponse::Pong),
        Interaction::Command(cmd) => handle_cmd(cmd),
        _ => Response::error("Interaction not implemented", 501),
    }
}
