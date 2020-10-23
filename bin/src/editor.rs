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

impl sl::uial::input::CustomAction for Action {
    fn parse_input(cmd: Option<String>) -> Result<SystemAction<Action>, String> {
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

impl<'a> sl::exec::Sim for State {
    type ActionEnum = Action;
    type Cfg = Cfg;
    type Store = sl::default::Store;
    type DisplayData = sl::default::DisplayData;
    fn resolve(
        &mut self,
        _cfg: &Cfg,
        _a: Action,
    ) -> Result<display::RenderMode<Self::DisplayData>, sl::exec::ResolutionErr> {
        // TODO This needs to be done at some point...
        Ok(sl::uial::display::RenderMode::Render(Default::default()))
    }
}

pub struct EditorRenTup<'a>(sl::default::RenderTup<'a, State>);

impl<'a> std::fmt::Display for EditorRenTup<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!("Rendering is not implemented for the editor.");
    }
}
