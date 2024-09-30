use rand::Rng;
use serde_discord::{
    interaction::CommandInteractionData,
    response::{data::MessageBuilder, InteractionResponse},
    types::MultiTypeValue,
};
use worker::Response;

pub fn roll(cmd: CommandInteractionData) -> Result<Response, worker::Error> {
    let mut min_val: i64 = 1;
    let mut max_val: i64 = 6;

    if let Some(opt) = cmd.option("min") {
        if let Some(val) = opt.value() {
            match val {
                MultiTypeValue::Integer(val) => min_val = *val,
                _ => (),
            }
        }
    }

    if let Some(opt) = cmd.option("max") {
        if let Some(val) = opt.value() {
            match val {
                MultiTypeValue::Integer(val) => max_val = *val,
                _ => (),
            }
        }
    }

    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(min_val..=max_val);

    let msg = MessageBuilder::new()
        .content(rand_number.to_string())
        .build();

    let resp = InteractionResponse::Message(msg);
    Response::from_json(&resp)
}
