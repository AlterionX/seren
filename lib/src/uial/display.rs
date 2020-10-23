#[derive(Debug)]
pub enum RenderMode<T> {
    Render(T),
    Ignore,
}

pub struct RenderTup<A, B, C>(pub A, pub B, pub C);

#[derive(Debug)]
pub enum Err {
    IO(std::io::Error),
}

impl From<std::io::Error> for Err {
    fn from(e: std::io::Error) -> Self {
        Err::IO(e)
    }
}

pub trait Display<State, Cfg, RenderData> {
    fn display(&mut self, content: &State, cfg: &Cfg, d: RenderData) -> Result<(), Err>;
}

pub struct CmdDisplay<State, Cfg, RenderData> {
    phantom: std::marker::PhantomData<(State, Cfg, RenderData)>,
}

impl<State, Cfg, RenderData> Display<State, Cfg, RenderData> for CmdDisplay<State, Cfg, RenderData>
    where for <'a> RenderTup<&'a State, &'a Cfg, RenderData>: std::fmt::Display
{
    fn display(&mut self, content: &State, cfg: &Cfg, d: RenderData) -> Result<(), Err> {
        println!("{}", RenderTup(content, cfg, d));
        Ok(())
    }
}

pub fn cmd_line<State, Cfg, RenderData>() -> CmdDisplay<State, Cfg, RenderData> {
    CmdDisplay {
        phantom: std::marker::PhantomData,
    }
}

pub struct RawCmdDisplay<State, Cfg, RenderData> {
    backup_display: CmdDisplay<State, Cfg, RenderData>,
    raw_term: Option<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl<State, Cfg, RenderData> Display<State, Cfg, RenderData> for RawCmdDisplay<State, Cfg, RenderData>
    where for <'a> RenderTup<&'a State, &'a Cfg, RenderData>: std::fmt::Display
{
    fn display(&mut self, content: &State, cfg: &Cfg, d: RenderData) -> Result<(), Err> {
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
            writeln!(raw_term, "{}", RenderTup(content, cfg, d))?;
            Ok(())
        } else {
            self.backup_display.display(content, cfg, d)
        }
    }
}

pub fn raw_cmd_line<State, Cfg, RenderData>() -> RawCmdDisplay<State, Cfg, RenderData> {
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
