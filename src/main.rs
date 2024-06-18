mod state;

use clap::Parser;
use state::{Action, State};
use std::io::{self, Write};

#[derive(Parser)]
struct Args;

fn main() -> anyhow::Result<()> {
    repl(Args::parse())
}

fn repl(Args {}: Args) -> anyhow::Result<()> {
    let mut state = State::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("{state}");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        if let Action::Terminate(result) = state.process(&input) {
            break result;
        }
    }
}
