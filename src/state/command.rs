use clap::Parser;

pub struct NotFound {
    pub invalid: String,
}

#[derive(Parser)]
pub struct Exit {
    #[arg(default_value_t = 0)]
    pub code: i32,
}

#[derive(Parser)]
pub struct Echo {
    pub message: Vec<String>,
}

pub enum Command {
    NotFound(NotFound),
    Exit(Exit),
    Echo(Echo),
}

impl Command {
    pub fn search(command: &str) -> anyhow::Result<Option<Self>> {
        const WHITESPACE: [char; 2] = [' ', '\t'];

        let command = command.trim();
        let (command_name, _) = command.split_once(WHITESPACE).unwrap_or((command, ""));
        let command = command.split(WHITESPACE);

        match command_name {
            "" => Ok(None),

            "exit" => Ok(Some(Self::Exit(Exit::try_parse_from(command)?))),

            "echo" => Ok(Some(Self::Echo(Echo::try_parse_from(command)?))),

            _ => Ok(Some(Self::NotFound(NotFound {
                invalid: command_name.into(),
            }))),
        }
    }
}
