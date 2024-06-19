mod command;

use command::Command;
use std::io::Write;

pub enum Action {
    Continue,
    Terminate(anyhow::Result<()>),
}

pub struct State {
    prompt: String,
    command: Option<Command>,
}

impl State {
    pub fn new() -> Self {
        Self {
            prompt: "$".into(),
            command: None,
        }
    }

    pub fn output(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        if let Some(command) = &self.command {
            match command {
                Command::NotFound { invalid } => writeln!(writer, "{invalid}: command not found")?,
            }
        }
        write!(writer, "{} ", self.prompt)?;
        Ok(())
    }

    pub fn process(&mut self, input: &str) -> Action {
        let (command, args) = input.split_once([' ', '\t', '\n']).unwrap_or((input, ""));
        self.command = Command::search(command);
        Action::Continue
    }
}
