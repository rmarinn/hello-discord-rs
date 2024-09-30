mod roll;

use roll::*;
use serde_discord::interaction::CommandInteractionData;
use worker::{console_debug, Response};

pub fn handle_cmd(cmd: CommandInteractionData) -> Result<Response, worker::Error> {
    console_debug!("GOT: {:?}", cmd);
    match cmd.name() {
        "roll" => Response::from_json(&roll(cmd)),
        _ => Response::error("Command not implemented", 501),
    }
}
