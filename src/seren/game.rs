use serde::{Serialize, Deserialize};
use std::{io::BufReader, fs::File};
use crate::{game::{self, input::SystemAction}, seren::lib::{cfg, scene, stats}};

pub use cfg::Cfg;

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

pub struct State {
    curr_scene_name: String,
    curr_scene: scene::StandardScene,
    curr_line: u64,
    stats: Option<stats::Stats>,
}
impl<'a> game::State for State {
    type ActionEnum = Action;
    fn resolve(&mut self, a: Action) -> game::Result<game::display::RenderMode> {
        Ok(game::display::RenderMode::Render)
    }
}

impl State {
    pub fn init(cfg: &cfg::Cfg) -> Result<State, game::LoadErr> {
        let p = cfg.scenes.join(cfg.primary_scene.as_str());
        let f = File::open(p)?;
        let buf = BufReader::new(f);
        Ok(State {
            curr_scene: serde_yaml::from_reader(buf)?,
            stats: None,
            curr_scene_name: "main_menu".to_owned(),
            curr_line: 0,
        })
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.curr_scene.get_line(self.curr_line as usize))
    }
}
