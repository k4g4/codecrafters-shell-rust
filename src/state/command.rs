use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};

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

pub enum Type {
    Builtin(String),
    Path(String, PathBuf),
    NotFound(String),
    None,
}

impl Type {
    fn new(command: Option<&str>, executables: &[DirEntry]) -> Self {
        match command {
            None => Self::None,
            Some("echo" | "exit" | "type") => Self::Builtin(command.unwrap().into()),
            Some(command) => {
                if let Some(executable) =
                    executables.iter().find(|exec| exec.file_name() == command)
                {
                    Self::Path(command.into(), executable.path())
                } else {
                    Self::NotFound(command.into())
                }
            }
        }
    }
}

pub enum Command {
    NotFound(NotFound),
    Exit(Exit),
    Echo(Echo),
    Type(Type),
    Path(PathBuf, Vec<String>),
}

impl Command {
    pub fn search(command: &str) -> anyhow::Result<Option<Self>> {
        const WHITESPACE: [char; 2] = [' ', '\t'];

        let executables = {
            let mut execs = vec![];
            for path in env::var("PATH")?.split(':').take(2) {
                if let Ok(read_dir) = fs::read_dir(path) {
                    println!("{path} is ok");
                    for entry in read_dir {
                        execs.push(entry?);
                    }
                }
            }
            execs
        };

        let command = command.trim();
        let (command_name, _) = command.split_once(WHITESPACE).unwrap_or((command, ""));
        let command = command.split(WHITESPACE);

        match command_name {
            "" => Ok(None),

            "exit" => Ok(Some(Self::Exit(Exit::try_parse_from(command)?))),

            "echo" => Ok(Some(Self::Echo(Echo::try_parse_from(command)?))),

            "type" => Ok(Some(Self::Type(Type::new(
                command.skip(1).next(),
                &executables,
            )))),

            _ => {
                if let Some(executable) = executables
                    .iter()
                    .find(|exec| exec.file_name() == command_name)
                {
                    Ok(Some(Self::Path(
                        executable.path(),
                        command.skip(1).map(Into::into).collect(),
                    )))
                } else {
                    Ok(Some(Self::NotFound(NotFound {
                        invalid: command_name.into(),
                    })))
                }
            }
        }
    }
}
