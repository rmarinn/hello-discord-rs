use rand::Rng;
use serde_discord::{
    interaction::CommandInteractionData,
    response::{data::MessageBuilder, InteractionResponse},
    types::MultiTypeValue,
};

pub fn roll(cmd: CommandInteractionData) -> InteractionResponse {
    let mut min_val: i64 = 1;
    let mut max_val: i64 = 6;

    if let Some(opt) = cmd.option("min") {
        if let Some(val) = opt.value() {
            match val {
                MultiTypeValue::Integer(val) => {
                    min_val = *val;
                }
                _ => (),
            }
        }
    }

    if let Some(opt) = cmd.option("max") {
        if let Some(val) = opt.value() {
            match val {
                MultiTypeValue::Integer(val) => {
                    max_val = *val;
                }
                _ => (),
            }
        }
    }

    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(min_val..=max_val);

    let msg = MessageBuilder::new()
        .content(rand_number.to_string())
        .build();

    InteractionResponse::Message(msg)
}

#[cfg(test)]
mod test {
    use core::panic;

    use serde_discord::{
        interaction::CommandInteractionData,
        response::InteractionResponse,
        types::{CommandOptionKind, MultiTypeValue},
    };

    use super::roll;

    #[test]
    fn can_roll_within_range() {
        // check if setting min/max works
        for i in 1..100 {
            let opt_min = CommandInteractionData::new(
                "min",
                CommandOptionKind::Integer,
                Some(MultiTypeValue::Integer(i)),
                None,
                None,
            );
            let opt_max = CommandInteractionData::new(
                "max",
                CommandOptionKind::Integer,
                Some(MultiTypeValue::Integer(i)),
                None,
                None,
            );
            let cmd = CommandInteractionData::new(
                "roll",
                CommandOptionKind::SubCommand,
                None,
                Some(vec![opt_min, opt_max]),
                None,
            );

            match roll(cmd) {
                InteractionResponse::Message(msg) => {
                    let num: i64 = msg.content().clone().unwrap().parse().unwrap();
                    assert_eq!(num, i);
                }
                _ => panic!("should be InteractionResposne::Message"),
            }
        }

        // check if default rolls from 1-6
        for _ in [1..100] {
            let cmd = CommandInteractionData::new(
                "roll",
                CommandOptionKind::SubCommand,
                None,
                None,
                None,
            );

            match roll(cmd) {
                InteractionResponse::Message(msg) => {
                    let num: u64 = msg.content().clone().unwrap().parse().unwrap();
                    assert!(num >= 1);
                    assert!(num <= 6);
                }
                _ => panic!("should be InteractionResposne::Message"),
            }
        }
    }
}
