use std::fmt;

pub enum Action {
    Continue,
    Terminate(anyhow::Result<()>),
}

pub struct State;

impl State {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&mut self, input: &str) -> Action {
        Action::Continue
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$ ")
    }
}
