mod state;

use clap::Parser;
use state::State;
use std::{
    io::{stdin, stdout, BufWriter, Write},
    process::exit,
};

pub enum Action {
    Continue,
    Terminate {
        code: i32,
        result: anyhow::Result<()>,
    },
}

#[derive(Parser)]
struct Args;

fn main() -> anyhow::Result<()> {
    repl(Args::parse())
}

fn repl(Args {}: Args) -> anyhow::Result<()> {
    let mut state = State::new();
    let (stdin, mut stdout) = (stdin(), BufWriter::new(stdout()));
    let mut input = String::new();

    loop {
        state.output(&mut stdout)?;
        stdout.flush()?;
        input.clear();
        stdin.read_line(&mut input)?;
        if let Action::Terminate { code, result } = state.process(&input) {
            if let Err(error) = result {
                eprintln!("{error}");
            }
            exit(code);
        }
    }
}
