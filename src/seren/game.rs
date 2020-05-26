use serde::{Serialize, Deserialize};
use std::{io::BufReader, fs::File};
use crate::{game::{self, input::SystemAction}, seren::lib::{cfg, scene, stats::{self, StatStore}}};

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
    curr_line: usize,
    stats: Option<stats::Stats>,
    error_text: Option<String>,
}

impl<'a> game::State for State {
    type ActionEnum = Action;
    type Cfg = Cfg;
    fn resolve(&mut self, cfg: &Cfg, a: Action) -> Result<game::display::RenderMode, game::Resolution> {
        let mut needs_scene_load = false;
        match a {
            Action::Progress => {
                let line = self.curr_scene.get_line(self.curr_line);
                match line {
                    Some(scene::StandardLineEnum::Plain { .. }) => {
                        if self.curr_scene.line_count() == 0 || self.curr_line < self.curr_scene.line_count() - 1 {
                            log::debug!("Progressing to next line.");
                            self.curr_line += 1;
                            log::trace!("Current line set to {}.", self.curr_line);
                        } else {
                            log::debug!("Reached end of scene.");
                            needs_scene_load = true;
                            if let Some(scene) = self.curr_scene.next_scene.as_ref() {
                                log::debug!("Progressing to next scene.");
                                self.curr_scene_name = scene.clone();
                                self.curr_line = 0;
                                log::trace!("Progressing to scene {}, line {}.", self.curr_scene_name, self.curr_line);
                            } else {
                                // TODO the game is over, what now?
                                log::debug!("Terminal scene completed. Restarting game.");
                                self.reset_no_load(cfg);
                            }
                        }
                    },
                    Some(scene::StandardLineEnum::Choice { .. }) => {
                        log::warn!("Attempted to skip option.");
                        return Ok(game::display::RenderMode::Ignore);
                    },
                    None => {
                        Err("OOB current line, what to do?".to_string())?;
                    },
                }
            },
            Action::Select(choice) => {
                let line = self.curr_scene.get_line(self.curr_line);
                match line {
                    Some(scene::StandardLineEnum::Plain { .. }) => {
                        log::warn!("Attempted to pick option when no options exist.");
                        return Ok(game::display::RenderMode::Ignore);
                    },
                    Some(line @ scene::StandardLineEnum::Choice { .. }) => {
                        let choices = scene::FilteredStandardLine {
                            line,
                            stats: self.stats.as_ref()
                        }.get_filtered_choices();
                        let scene::Choice {
                            stat_changes,
                            scene_change,
                            ..
                        } = choices.get(choice)
                            .ok_or_else(|| "Attmpted to pick nonexistent option.".to_string())?;
                        self.curr_line += 1;
                        if let (Some(stats), Some(changes)) = (self.stats.as_mut(), stat_changes.as_ref()) {
                            for change in changes {
                                stats.apply(change);
                            }
                        }
                        if let Some(change) = scene_change {
                            let scene::SceneChange {
                                target_scene,
                                target_line,
                            } = change;
                            if let Some(scene_name) = target_scene {
                                let scene_name = scene_name.clone();
                                self.curr_scene_name = scene_name.clone();
                                self.curr_line = 0;
                                needs_scene_load = true;
                            }
                            if let Some(line) = target_line {
                                self.curr_line = line.clone();
                            }
                        }
                    },
                    None => {
                        Err("OOB current line, what to do?".to_string())?;
                    },
                }
            },
        };
        if needs_scene_load {
            log::debug!("New scene required. Loading...");
            let scene = Self::load_scene(cfg, self.curr_scene_name.as_str())
                .map_err(|_| "Failed to load scene.".to_string())?;
            self.curr_scene = scene;
        }
        Ok(game::display::RenderMode::Render)
    }
}

impl State {
    pub fn init(cfg: &cfg::Cfg) -> Result<State, game::LoadErr> {
        let scene = Self::load_scene(cfg, cfg.primary_scene.as_str())?;
        Ok(State {
            curr_scene: scene,
            stats: None,
            curr_scene_name: cfg.primary_scene.clone(),
            curr_line: 0,
            error_text: None,
        })
    }

    fn reset_no_load(&mut self, cfg: &cfg::Cfg) {
        self.stats = None;
        self.curr_scene_name = cfg.primary_scene.clone(); 
        self.curr_line = 0;
        self.error_text = None;
    }

    fn load_scene(cfg: &Cfg, name: &str) -> Result<scene::StandardScene, game::LoadErr> {
        log::error!("Loading scene {:?}.", name);
        let p = cfg.root.join(cfg.scenes.as_path()).join(format!("{}.yaml", name));
        log::debug!("Loading scene from file {}.", p.display());
        let f = File::open(p)?;
        let buf = BufReader::new(f);
        Ok(serde_yaml::from_reader(buf)?)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(line) = self.curr_scene.get_line(self.curr_line) {
            write!(f, "{}", scene::FilteredStandardLine { line, stats: self.stats.as_ref() })
        } else {
            write!(f, "Oops, an error has occurred.")
        }
    }
}
