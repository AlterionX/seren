pub trait Display<T, Cfg> {
    fn display(&mut self, content: &T, cfg: &Cfg);
}

pub struct CmdDisplay<State, Cfg> {
    phantom: std::marker::PhantomData<(State, Cfg)>
}

impl <T: std::fmt::Display, Cfg> Display<T, Cfg> for CmdDisplay<T, Cfg> {
    fn display(&mut self, content: &T, cfg: &Cfg) {
        println!("{}", content);
    }
}

pub fn cmd_line<State, Cfg>() -> CmdDisplay<State, Cfg> {
    CmdDisplay {
        phantom: std::marker::PhantomData
    }
}