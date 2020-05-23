use serde::{Serialize, Deserialize};
use std::{path::{PathBuf, Path}, io::BufReader, fs::File};
use crate::game::{self, input::SystemAction};

mod scene;

mod inventory;
mod item;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cfg {
    scene_list: Vec<String>,
    scenes: PathBuf,
    primary_scene: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Progress,
    Select(usize)
}

pub fn parse_cfg(p: &Path) -> Result<Cfg, game::InitErr> {
    // TODO parse path
    let f = File::open(p.join("cfg.yaml"))?;
    let buf = BufReader::new(f);
    Ok(serde_yaml::from_reader(buf)?)
}

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


struct Stats {
    bossiness: usize,
}

pub struct State {
    curr_scene_name: String,
    curr_scene: scene::StandardScene,
    curr_line: u64,
    stats: Option<Stats>,
}
impl<'a> game::State for State {
    type ActionEnum = Action;
    fn resolve(&mut self, a: Action) -> game::Result<()> {
        Ok(())
    }
}
pub fn init_state(cfg: &Cfg) -> Result<State, game::LoadErr> {
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

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.curr_scene.get_line(self.curr_line as usize))
    }
}