mod state;

use clap::Parser;
use state::{Action, State};
use std::io::{stdin, stdout, BufWriter, Write};

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
        if let Action::Terminate(result) = state.process(&input) {
            break result;
        }
    }
}
