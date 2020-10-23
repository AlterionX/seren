use crate::{
    game::{self, input::SystemAction},
    seren::lib::{
        cfg, scene,
        stats::{self, StatStore},
    },
    util::Boo,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

pub use cfg::Cfg;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Progress,
    PromptRetry,
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
                        if n == 0 {
                            Action::PromptRetry
                        } else {
                            Action::Select(n - 1)
                        }
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
    curr_line: usize,
    stats: Option<stats::Stats>,
    error_text: Option<String>,
}

struct StateChange<'a> {
    stat_changes: Option<Vec<&'a scene::StatChange<stats::Stat>>>,
    scene_change: Option<&'a scene::SceneChange>,
}

impl<'a> StateChange<'a> {
    fn merge<'b: 'a, 'c: 'a>(lhs: StateChange<'b>, rhs: StateChange<'c>) -> Self {
        let stat_changes = match (lhs.stat_changes, rhs.stat_changes) {
            (None, stat_changes) | (stat_changes, None) => stat_changes,
            (Some(mut lhs), Some(rhs)) => {
                lhs.extend(rhs);
                Some(lhs)
            }
        };
        let scene_change = match (lhs.scene_change, rhs.scene_change) {
            (_, scene_change @ Some(_)) | (scene_change, None) => scene_change,
        };
        Self {
            stat_changes,
            scene_change,
        }
    }
}

impl State {
    fn load_scene(cfg: &Cfg, name: &str) -> Result<scene::StandardScene, game::LoadErr> {
        log::debug!("Loading scene {:?}.", name);
        let p = cfg
            .root
            .join(cfg.scenes.as_path())
            .join(format!("{}.yaml", name));
        log::debug!("Loading scene from file {}.", p.display());
        let f = File::open(p)?;
        let buf = BufReader::new(f);
        Ok(serde_yaml::from_reader(buf)?)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(text) = self.error_text.as_ref() {
            write!(f, "Error encountered: {}", text)?;
        }
        // Display the line, even if there was an error.
        if let Some(line) = self.curr_scene.get_line(self.curr_line) {
            let line = &line.line;
            write!(
                f,
                "{}",
                sl::game::scene::FilteredStandardLine {
                    line,
                    stats: self.stats.as_ref()
                }
            )
        } else {
            write!(f, "Oops, an error has occurred.")
        }
    }
}
