use rand::Rng;
use worker::Response;

use crate::{CommandInteractionData, InteractionResponse, MessageBuilder};

pub fn roll(cmd: CommandInteractionData) -> Result<Response, worker::Error> {
    let mut sides: i64 = 6;
    if let Some(opt) = cmd.option("sides") {
        if let Some(val) = opt.value() {
            match val {
                crate::MultiTypeValue::Integer(val) => sides = *val,
                _ => (),
            }
        }
    }
    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(1..=sides);
    let msg = MessageBuilder::new()
        .content(rand_number.to_string())
        .build();
    let resp = InteractionResponse::Message(msg);
    Response::from_json(&resp)
}
