#[derive(Debug)]
pub enum RenderMode<T> {
    Render(T),
    Ignore,
}

pub trait RenderGroup<'a, A, B, C> {
    fn create(a: &'a A, b: &'a B, c: C) -> Self;
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

pub trait Display<A, B, C, R> {
    fn display(&mut self, content: &A, cfg: &B, d: C) -> Result<(), Err>;
}

pub struct CmdDisplay<A, B, C, R> {
    phantom: std::marker::PhantomData<(A, B, C, R)>,
}

impl<A, B, C, R> Display<A, B, C, R> for CmdDisplay<A, B, C, R>
    where R: for <'a> RenderGroup<'a, A, B, C> + std::fmt::Display
{
    fn display(&mut self, a: &A, b: &B, c: C) -> Result<(), Err> {
        println!("{}", R::create(a, b, c));
        Ok(())
    }
}

pub fn cmd_line<A, B, C, R>() -> CmdDisplay<A, B, C, R> {
    CmdDisplay {
        phantom: std::marker::PhantomData,
    }
}

pub struct RawCmdDisplay<A, B, C, R> {
    backup_display: CmdDisplay<A, B, C, R>,
    raw_term: Option<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl<A, B, C, R> Display<A, B, C, R> for RawCmdDisplay<A, B, C, R>
    where R: for <'a> RenderGroup<'a, A, B, C> + std::fmt::Display
{
    fn display(&mut self, a: &A, b: &B, c: C) -> Result<(), Err> {
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
            writeln!(raw_term, "{}", R::create(a, b, c))?;
            Ok(())
        } else {
            self.backup_display.display(a, b, c)
        }
    }
}

pub fn raw_cmd_line<A, B, C, R>() -> RawCmdDisplay<A, B, C, R> {
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
