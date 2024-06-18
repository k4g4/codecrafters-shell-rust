use std::io::Write;

pub enum Action {
    Continue,
    Terminate(anyhow::Result<()>),
}

pub struct State {
    prompt: String,
}

impl State {
    pub fn new() -> Self {
        Self { prompt: "$".into() }
    }

    pub fn output(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        write!(writer, "{} ", self.prompt)?;

        Ok(())
    }

    pub fn process(&mut self, input: &str) -> Action {
        Action::Continue
    }
}
