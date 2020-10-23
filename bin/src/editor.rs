// This entire file is TODO.

use sl::uial::{input::SystemAction, display};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    game_cfg: sl::game::Cfg,
}

impl State {
    pub fn new(cfg: sl::game::Cfg) -> State {
        State { game_cfg: cfg }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cfg;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Progress,
    Select(usize),
}

impl Action {
    pub fn parse_input(cmd: Option<String>) -> Result<SystemAction<Action>, String> {
        let action = if let Some(cmd) = cmd {
            log::debug!("Entry echo: {:?}", cmd);
            let action = match cmd.as_str() {
                "" => Action::Progress,
                _ => {
                    if let Some(n) = cmd.parse::<usize>().ok() {
                        Action::Select(n)
                    } else {
                        // TODO eventually consider some sort of arbitrary text input.
                        Action::Progress
                    }
                }
            };
            SystemAction::Action(action)
        } else {
            SystemAction::Exit
        };
        Ok(action)
    }
}

impl<'a> sl::exec::State for State {
    type ActionEnum = Action;
    type Cfg = Cfg;
    fn resolve(
        &mut self,
        _cfg: &Cfg,
        _a: Action,
    ) -> sl::SeRes<display::RenderMode, sl::exec::ResolutionErr> {
        // TODO This needs to be done at some point...
        Ok(game::display::RenderMode::Render)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "hello")
    }
}
