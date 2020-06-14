use serde::{Serialize, Deserialize};
use std::{io::BufReader, fs::File};
use tap::*;
use crate::{game::{self, input::SystemAction}, seren::lib::{cfg, scene, stats::{self, StatStore}}};

pub use cfg::Cfg;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                "" => {
                    Action::Progress
                }
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

enum BoO<'a, T> {
    Borrowed(&'a T),
    Owned(Box<T>),
}

impl<'a, T> BoO<'a, T> {
    fn borrowed(self) -> Option<&'a T> {
        match self {
            Self::Owned(_) => None,
            Self::Borrowed(t) => Some(t),
        }
    }
    fn owned(self) -> Option<T> {
        match self {
            Self::Borrowed(_) => None,
            Self::Owned(t) => Some(*t),
        }
    }
}

impl<'a, T> From<T> for BoO<'a, T> {
    fn from(t: T) -> Self {
        Self::Owned(Box::new(t))
    }
}

impl<'a, T> From<&'a T> for BoO<'a, T> {
    fn from(t: &'a T) -> Self {
        Self::Borrowed(t)
    }
}

impl<'a, T> AsRef<T> for BoO<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            BoO::Borrowed(t) => *t,
            BoO::Owned(t) => t,
        }
    }
}

impl<'a, T> std::ops::Deref for BoO<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

struct BorrowedMutState<'a> {
    curr_scene_name: &'a mut String,
    curr_line: &'a mut usize,
    stats: &'a mut Option<stats::Stats>,
    error_text: &'a mut Option<String>,
}

struct StateChange<'a> {
    stat_changes: Option<&'a [scene::StatChange<stats::Stat>]>,
    scene_change: Option<&'a scene::SceneChange>,
}

type SceneBoO<'a> = BoO<'a, scene::StandardScene>;
impl<'a> BorrowedMutState<'a> {
    fn progress_line(&mut self, cfg: &Cfg, scene: SceneBoO, bypass_initial_increment: bool) -> Result<Option<scene::StandardScene>, game::Resolution> {
        let line_count = scene.line_count();
        if bypass_initial_increment {
            log::debug!("Bypassing line increment.");
            Ok(scene.owned())
        } else if line_count != 0 && (*self.curr_line) < line_count - 1 {
            log::debug!("Progressing to next line.");
            (*self.curr_line) += 1;
            log::trace!("Current line set to {}.", self.curr_line);
            Ok(scene.owned())
        } else {
            log::debug!("Reached end of scene.");
            if let Some(scene) = scene.next_scene.as_ref() {
                log::debug!("Progressing to next scene.");
                (*self.curr_scene_name) = scene.clone();
                (*self.curr_line) = 0;
                log::trace!("Progressing to scene {}, line {}.", self.curr_scene_name, self.curr_line);
            } else {
                // TODO the game is over, what now? For now, it just restarts.
                log::debug!("Terminal scene completed. Restarting game.");
                self.reset_no_load(cfg);
            }

            let scene = State::load_scene(cfg, self.curr_scene_name.as_str())
                .map_err(|e| format!("Failed to load scene {} after processing choice.", self.curr_scene_name))?;
            Ok(Some(scene))
        }
    }

    fn apply_changes_and_progress_line(&mut self, cfg: &Cfg, scene: &scene::StandardScene, StateChange {
        stat_changes,
        scene_change,
    }: StateChange<'a>) -> Result<Option<scene::StandardScene>, game::Resolution> {
        let mut needs_scene_load = false;
        if let (Some(stats), Some(changes)) = (self.stats.as_mut(), stat_changes) {
            log::info!("Applying stat changes.");
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
                (*self.curr_scene_name) = scene_name.clone();
                log::info!("Setting target scene to {:?}, at line 0 while processing choice selection.", self.curr_scene_name);
                (*self.curr_line) = 0;
                needs_scene_load = true;
            }
            if let Some(line) = target_line {
                log::info!("Line number changed from {} to {} while processing choice selection.", self.curr_line, line);
                (*self.curr_line) = *line;
            }
        }

        let scene = if needs_scene_load {
            let scene = State::load_scene(cfg, self.curr_scene_name.as_str())
                .map_err(|e| format!("Failed to load scene {} after processing choice.", self.curr_scene_name))?;
            SceneBoO::from(scene)
        } else {
            SceneBoO::from(scene)
        };
        self.progress_line(cfg, scene, needs_scene_load)
    }

    fn reset_no_load(&mut self, cfg: &cfg::Cfg) {
        let _ = (*self.stats).take();
        (*self.curr_scene_name) = cfg.primary_scene.clone(); 
        (*self.curr_line) = 0;
        let _ = (*self.error_text).take();
    }
}

impl State {
    fn split_scene<'a>(&'a mut self) -> (BorrowedMutState<'a>, &scene::StandardScene) {
        (BorrowedMutState {
            curr_line: &mut self.curr_line,
            curr_scene_name: &mut self.curr_scene_name,
            stats: &mut self.stats,
            error_text: &mut self.error_text,
        }, &self.curr_scene)
    }
}

impl<'a> game::State for State {
    type ActionEnum = Action;
    type Cfg = Cfg;
    fn resolve(&mut self, cfg: &Cfg, a: Action) -> Result<game::display::RenderMode, game::Resolution> {
        let _ = self.error_text.take();
        let (mut borrowed_self, scene) = self.split_scene();
        let line = scene.get_line(*borrowed_self.curr_line);

        let next_scene = match (a, line) {
            // Error handling
            (Action::PromptRetry, _) => {
                let _ = self.error_text.replace("Invalid input. Please retry.".to_string());
                None
            },
            (_, None) => {
                log::error!("game state has invalid state, current line {} does not exist in scene {}", self.curr_line, self.curr_scene_name);
                return Err("OOB Line. Early termination.".to_owned().into());
            },
            // Player actions
            (Action::Progress, Some(scene::StandardLineEnum::Plain { .. })) => {
                borrowed_self.progress_line(cfg, SceneBoO::from(scene), false)?
            },
            (Action::Progress, Some(scene::StandardLineEnum::Choice {
                default_choice: None,
                ..
            })) => {
                log::warn!("Attempted to skip option.");
                return Ok(game::display::RenderMode::Ignore)
            },
            (Action::Progress, Some(scene::StandardLineEnum::Choice {
                default_choice: Some(default_choice),
                choices,
                ..
            })) => {
                log::info!("Attempted to select default choice.");
                let stats = borrowed_self.stats.as_ref().ok_or_else(|| stats::Stats::default());
                let stats = match &stats {
                    Ok(stats) => *stats,
                    Err(stats) => stats,
                };

                let default_choice = if let Some(default_choice) = choices.get(*default_choice) {
                    default_choice
                } else {
                    return Err("Default choice does not exists".to_owned().into());
                };

                let guards = if let Some(guards) = default_choice.guards.as_ref() {
                    guards.as_slice()
                } else {
                    &[]
                };

                if guards.iter().all(|guard| stats.verify(guard)) {
                    let scene::Choice {
                        stat_changes,
                        scene_change,
                        ..
                    } = default_choice;
                    borrowed_self.apply_changes_and_progress_line(cfg, scene, StateChange {
                        stat_changes: stat_changes.as_ref().map(|v| v.as_slice()),
                        scene_change: scene_change.as_ref(),
                    })?
                } else {
                    log::warn!("Attempted to skip option.");
                    return Ok(game::display::RenderMode::Ignore);
                }
            },
            (Action::Select(_), Some(scene::StandardLineEnum::Plain { .. })) => {
                log::warn!("Attempted to pick option when no options exist.");
                return Ok(game::display::RenderMode::Ignore);
            },
            (Action::Select(choice), Some(line @ scene::StandardLineEnum::Choice { .. })) => {
                log::info!("User selected option number {}.", choice);
                let choices = scene::FilteredStandardLine {
                    line,
                    stats: borrowed_self.stats.as_ref()
                }.get_filtered_choices();
                let scene::Choice {
                    stat_changes,
                    scene_change,
                    ..
                } = if let Some(c) = choices.get(choice) {
                    log::trace!("User selected option {:?}.", c);
                    c
                } else {
                    self.error_text.replace("Attmpted to pick nonexistent option.".to_string());
                    return Ok(game::display::RenderMode::Render);
                };
                borrowed_self.apply_changes_and_progress_line(cfg, scene, StateChange {
                    stat_changes: stat_changes.as_ref().map(|v| v.as_slice()),
                    scene_change: scene_change.as_ref(),
                })?
            },
        };

        if let Some(next_scene) = next_scene {
            self.curr_scene = next_scene;
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

    fn load_scene(cfg: &Cfg, name: &str) -> Result<scene::StandardScene, game::LoadErr> {
        log::debug!("Loading scene {:?}.", name);
        let p = cfg.root.join(cfg.scenes.as_path()).join(format!("{}.yaml", name));
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
            write!(f, "{}", scene::FilteredStandardLine { line, stats: self.stats.as_ref() })
        } else {
            write!(f, "Oops, an error has occurred.")
        }
    }
}
