#[derive(Debug)]
pub enum RenderMode {
    Render,
    Ignore,
}

#[derive(Debug)]
pub enum Err {
    IO(std::io::Error),
}

impl From<std::io::Error> for Err {
    fn from(e: std::io::Error) -> Self {
        Err::IO(e)
    }
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

pub struct RawCmdDisplay<State, Cfg> {
    backup_display: CmdDisplay<State, Cfg>,
    raw_term: Option<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl <State: std::fmt::Display, Cfg> Display<State, Cfg> for RawCmdDisplay<State, Cfg> {
    fn display(&mut self, content: &State, cfg: &Cfg) -> Result<(), Err> {
        if let Some(raw_term) = self.raw_term.as_mut() {
            use std::io::Write;
            raw_term.activate_raw_mode()?;
            write!(
                raw_term,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
            )?;
            raw_term.flush()?;
            raw_term.suspend_raw_mode()?;
            writeln!(raw_term, "{}", content)?;
            Ok(())
        } else {
            self.backup_display.display(content, cfg)
        }
    }
}

pub fn raw_cmd_line<State, Cfg>() -> RawCmdDisplay<State, Cfg> {
    use termion::raw::IntoRawMode;
    let raw_term = std::io::stdout()
        .into_raw_mode()
        .and_then(|term| {
            term.suspend_raw_mode()?;
            Ok(term)
        })
        .ok();
    // Note that the terminal should never be in raw mode at this point,
    // since either the `RawTerminal` is dropped, causing stdout to recover,
    // or the `suspend_raw_mode` call succeeds, causing stdout to not be in
    // raw mode, or the initial `into_raw_mode` call failed, and we never
    // set stdout to raw mode in the first place.
    RawCmdDisplay {
        backup_display: cmd_line(),
        raw_term,
    }
}