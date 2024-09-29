mod roll;

use roll::*;
use worker::Response;

use crate::CommandInteractionData;

pub fn handle_cmd(cmd: CommandInteractionData) -> Result<Response, worker::Error> {
    match cmd.name() {
        "roll" => roll(cmd),
        _ => Response::error("Command not implemented", 501),
    }
}
