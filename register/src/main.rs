use register::*;

fn make_cmd_blep() -> Command {
    let choice_dog = CommandOptionChoiceBuilder::new()
        .name("Dog")
        .value(OptionValue::String("animal_dog".into()))
        .build()
        .unwrap();
    let choice_cat = CommandOptionChoiceBuilder::new()
        .name("Cat")
        .value(OptionValue::String("animal_cat".into()))
        .build()
        .unwrap();
    let choice_penguin = CommandOptionChoiceBuilder::new()
        .name("Penguin")
        .value(OptionValue::String("animal_penguin".into()))
        .build()
        .unwrap();
    let animal_opts = CommandOptionBuilder::new()
        .name("animal")
        .description("The type of animal")
        .kind(CommandOptionKind::String)
        .required(true)
        .set_choices(vec![choice_dog, choice_cat, choice_penguin])
        .build()
        .unwrap();
    let only_smol = CommandOptionBuilder::new()
        .name("only_smol")
        .description("Whether to show only baby animals")
        .kind(CommandOptionKind::Boolean)
        .required(true)
        .build()
        .unwrap();
    let options = vec![animal_opts, only_smol];

    let cmd = CommandBuilder::new()
        .name("blep")
        .kind(CommandKind::ChatInput)
        .description("Send a random adorable animal photo")
        .set_options(options)
        .build()
        .unwrap();

    cmd
}

fn make_cmd_roll() -> Command {
    let opts = CommandOptionBuilder::new()
        .name("sides")
        .description("The number of sides of the die")
        .kind(CommandOptionKind::Integer)
        .min_value(OptionValue::Int(1))
        .max_value(OptionValue::Int(i32::MAX))
        .build()
        .unwrap();
    let cmd = CommandBuilder::new()
        .name("roll")
        .kind(CommandKind::ChatInput)
        .description("Roll a die")
        .option(opts)
        .build()
        .unwrap();
    cmd
}

#[tokio::main]
async fn main() {
    let client = Client::new().unwrap();

    let cmd_blep = make_cmd_blep();
    let cmd_roll = make_cmd_roll();
    let cmds = vec![cmd_blep, cmd_roll];

    client.register_commands(cmds).await;
}
