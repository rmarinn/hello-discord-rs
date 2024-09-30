use register::*;
use serde_discord::{
    register::{ChoiceValue, Command, CommandBuilder, CommandOptionBuilder},
    types::{CommandKind, CommandOptionKind},
};

fn make_cmd_roll() -> Command {
    let opt_min = CommandOptionBuilder::new()
        .name("min")
        .description("The mininum number to roll")
        .kind(CommandOptionKind::Integer)
        .min_value(ChoiceValue::Int(0))
        .max_value(ChoiceValue::Int(i32::MAX - 1))
        .build()
        .unwrap();
    let opt_max = CommandOptionBuilder::new()
        .name("max")
        .description("The max number to roll")
        .kind(CommandOptionKind::Integer)
        .min_value(ChoiceValue::Int(1))
        .max_value(ChoiceValue::Int(i32::MAX))
        .build()
        .unwrap();
    let cmd = CommandBuilder::new()
        .name("roll")
        .kind(CommandKind::ChatInput)
        .description("Roll a die")
        .options(vec![opt_min, opt_max])
        .build()
        .unwrap();
    cmd
}

#[tokio::main]
async fn main() {
    let client = Client::new().unwrap();

    let cmd_roll = make_cmd_roll();
    let cmds = vec![cmd_roll];

    client.register_commands(cmds).await;
}
