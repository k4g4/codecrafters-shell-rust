pub enum Command {
    NotFound { invalid: String },
}

impl Command {
    pub fn search(command: &str) -> Option<Command> {
        (!command.is_empty()).then(|| Command::NotFound {
            invalid: command.into(),
        })
    }
}
