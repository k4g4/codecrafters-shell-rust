mod command;

use super::Action;
use command::{Command, Echo, Exit, NotFound, Type};
use std::{env, io::Write, process};

struct Settings {
    prompt: String,
}

enum CommandState {
    Empty,
    Command(Command),
    Error(anyhow::Error),
}

pub struct State {
    settings: Settings,
    command_state: CommandState,
}

impl State {
    pub fn new() -> Self {
        Self {
            settings: Settings { prompt: "$".into() },
            command_state: CommandState::Empty,
        }
    }

    pub fn output(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        match &self.command_state {
            CommandState::Empty => {}

            CommandState::Command(command) => match command {
                Command::NotFound(NotFound { invalid }) => {
                    writeln!(writer, "{invalid}: command not found")?;
                }

                Command::Exit(_) => unreachable!("should have exited before reaching here"),

                Command::Echo(Echo { message }) => {
                    if !message.is_empty() {
                        for word in &message[..message.len() - 1] {
                            write!(writer, "{word} ")?;
                        }
                        writeln!(writer, "{}", message.last().unwrap())?;
                    }
                }

                Command::Pwd(_) => writeln!(writer, "{}", env::current_dir()?.display())?,

                Command::Type(r#type) => match r#type {
                    Type::Builtin(command) => writeln!(writer, "{command} is a shell builtin")?,
                    Type::NotFound(command) => writeln!(writer, "{command}: not found")?,
                    Type::Path(command, path) => {
                        writeln!(writer, "{command} is {}", path.display())?
                    }
                    Type::None => {}
                },

                Command::Path(path, args) => write!(
                    writer,
                    "{}",
                    String::from_utf8_lossy(
                        &process::Command::new(path).args(args).output()?.stdout
                    )
                )?,
            },

            CommandState::Error(error) => writeln!(writer, "{error}")?,
        }
        write!(writer, "{} ", self.settings.prompt)?;
        Ok(())
    }

    pub fn process(&mut self, input: &str) -> Action {
        match Command::search(input) {
            Ok(None) => {
                self.command_state = CommandState::Empty;
                Action::Continue
            }

            Ok(Some(command)) => {
                if let Command::Exit(Exit { code }) = command {
                    Action::Terminate {
                        code,
                        result: if code == 0 {
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!("Exiting due to error code: {code}"))
                        },
                    }
                } else {
                    self.command_state = CommandState::Command(command);
                    Action::Continue
                }
            }

            Err(error) => {
                self.command_state = CommandState::Error(error);
                Action::Continue
            }
        }
    }
}
