#[derive(Debug)]
pub enum RenderMode {
    Render,
    Ignore,
}

#[derive(Debug)]
pub enum Err {
    IO(std::io::Error),
}

pub trait Display<State, Cfg> {
    fn display(&mut self, content: &State, cfg: &Cfg) -> Result<(), Err>;
}

pub struct CmdDisplay<State, Cfg> {
    phantom: std::marker::PhantomData<(State, Cfg)>
}

impl <State: std::fmt::Display, Cfg> Display<State, Cfg> for CmdDisplay<State, Cfg> {
    fn display(&mut self, content: &State, _cfg: &Cfg) -> Result<(), Err> {
        println!("{}", content);
        Ok(())
    }
}

pub fn cmd_line<State, Cfg>() -> CmdDisplay<State, Cfg> {
    CmdDisplay {
        phantom: std::marker::PhantomData
    }
}