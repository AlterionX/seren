use serde::{Serialize, Deserialize};
use crate::{seren::lib::cfg, game::{self, input::SystemAction}};

#[derive(Serialize, Deserialize)]
pub struct State {
    game_cfg: cfg::Cfg,
}

impl State {
    pub fn new(cfg: cfg::Cfg) -> State {
        State {
            game_cfg: cfg,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cfg;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Progress,
    Select(usize)
}

impl Action {
    pub fn parse_input(cmd: Option<String>) -> Result<SystemAction<Action>, String> {
        let action = if let Some(cmd) = cmd {
            println!("Entry echo: {:?}", cmd);
            let action = match cmd.as_str() {
                "" => {
                    Action::Progress
                }
                _ => {
                    if let Some(n) = cmd.parse::<usize>().ok() {
                        Action::Select(n)
                    } else {
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

impl<'a> game::State for State {
    type ActionEnum = Action;
    fn resolve(&mut self, a: Action) -> game::Result<game::display::RenderMode> {
        Ok(game::display::RenderMode::Render)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "hello")
    }
}
