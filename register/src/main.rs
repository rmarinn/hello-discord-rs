use std::{env, error::Error};

use anyhow::{Context, Result};
use serde_discord::{
    register::{register_commands, ChoiceValue, Command, CommandBuilder, CommandOptionBuilder},
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
        .expect("Failed to build 'min' option");

    let opt_max = CommandOptionBuilder::new()
        .name("max")
        .description("The max number to roll")
        .kind(CommandOptionKind::Integer)
        .min_value(ChoiceValue::Int(1))
        .max_value(ChoiceValue::Int(i32::MAX))
        .build()
        .expect("Failed to build 'max' option");

    let cmd = CommandBuilder::new()
        .name("roll")
        .kind(CommandKind::ChatInput)
        .description("Roll a die")
        .options(vec![opt_min, opt_max])
        .build()
        .expect("Failed to build 'roll' command");

    cmd
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env_vars = load_env_vars()?;

    let cmd_roll = make_cmd_roll();
    let cmds = vec![cmd_roll];

    register_commands(&env_vars.app_id, &env_vars.token, cmds).await?;

    Ok(())
}

struct Env {
    app_id: String,
    token: String,
}

fn load_env_vars() -> Result<Env> {
    dotenvy::dotenv().context("Could not load environment variables")?; // Add context to the error

    let app_id =
        env::var("DISCORD_APP_ID").context("Environment variable `DISCORD_APP_ID` is not set")?;

    let token =
        env::var("DISCORD_TOKEN").context("Environment variable `DISCORD_TOKEN` is not set")?;

    Ok(Env { app_id, token })
}
