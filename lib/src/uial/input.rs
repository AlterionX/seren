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

pub trait CustomAction: Sized {
    fn parse_input(cmd: Option<String>) -> Result<SystemAction<Self>, String>;
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
    stdin: *const std::io::Stdin,
    lines: std::io::Lines<std::io::StdinLock<'a>>,
    _phantom: std::marker::PhantomData<Action>,
}

impl<'a, Action: CustomAction> Input<Action> for CmdInput<'a, Action> {
    fn next_action(&mut self) -> Result<SystemAction<Action>, Err> {
        let line = self.lines.next().transpose()?;
        Ok(Action::parse_input(line)?)
    }
}

pub fn cmd_line<'a, Action: CustomAction>() -> CmdInput<'a, Action> {
    use std::io::BufRead;
    // Self referential struct...?
    let stdin = Box::leak(Box::new(std::io::stdin()));
    let lines = stdin.lock().lines();
    CmdInput {
        stdin,
        lines,
        _phantom: std::marker::PhantomData,
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

impl<'a, Action: CustomAction> Input<Action> for RawCmdInput<'a, Action> {
    fn next_action(&mut self) -> Result<SystemAction<Action>, Err> {
        // TODO read the raw terminal inputs, since this won't work if raw mode is enabled.
        self.backup_input.next_action()
    }
}

pub fn raw_cmd_line<'a, Action: CustomAction>() -> RawCmdInput<'a, Action> {
    RawCmdInput {
        backup_input: cmd_line(),
    }
}
