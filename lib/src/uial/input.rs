#[derive(Debug)]
pub enum Err {
    IOErr(std::io::Error),
    OtherErr(String),
}
impl From<std::io::Error> for Err {
    fn from(e: std::io::Error) -> Self {
        Err::IOErr(e)
    }
}
impl From<String> for Err {
    fn from(e: String) -> Self {
        Err::OtherErr(e)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemAction<A> {
    Exit,
    Action(A),
}

pub trait Input<Action> {
    fn next_action(&mut self) -> Result<SystemAction<Action>, Err>;
}

pub struct CmdInput<'a, Action> {
    parse: fn(Option<String>) -> Result<SystemAction<Action>, String>,
    stdin: *const std::io::Stdin,
    lines: std::io::Lines<std::io::StdinLock<'a>>,
}

impl<'a, Action> Input<Action> for CmdInput<'a, Action> {
    fn next_action(&mut self) -> Result<SystemAction<Action>, Err> {
        let line = self.lines.next().transpose()?;
        let parse = &self.parse;
        Ok((parse(line))?)
    }
}

pub fn cmd_line<'a, Action>(
    parse: fn(Option<String>) -> Result<SystemAction<Action>, String>,
) -> CmdInput<'a, Action> {
    use std::io::BufRead;
    // Self referential struct...?
    let stdin = Box::leak(Box::new(std::io::stdin()));
    let lines = stdin.lock().lines();
    CmdInput {
        parse: parse,
        stdin: stdin,
        lines: lines,
    }
}

impl<'a, Action> Drop for CmdInput<'a, Action> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.stdin as *mut std::io::Stdin);
        }
    }
}

pub struct RawCmdInput<'a, Action> {
    backup_input: CmdInput<'a, Action>,
}

impl<'a, Action> Input<Action> for RawCmdInput<'a, Action> {
    fn next_action(&mut self) -> Result<SystemAction<Action>, Err> {
        // TODO read the raw terminal inputs, since this won't work if raw mode is enabled.
        self.backup_input.next_action()
    }
}

pub fn raw_cmd_line<'a, Action>(
    parse: fn(Option<String>) -> Result<SystemAction<Action>, String>,
) -> RawCmdInput<'a, Action> {
    RawCmdInput {
        backup_input: cmd_line(parse),
    }
}
