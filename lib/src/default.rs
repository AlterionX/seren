use crate::{uial, exec, game::{self, KeyedStore}};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::RangeBounds};

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    bools: HashMap<String, bool>,
    ints: HashMap<String, i64>,
    nums: HashMap<String, f64>,
}

impl game::KeyedStore for Store {
    type KeyEnum = String;

    fn check_keyed_guard(&self, game::guard::KeyedGuard { name, value, .. }: &game::guard::KeyedGuard<Self>) -> bool {
        match value {
            game::guard::ValueGuard::Bool(b) => {
                self.bools.get(name.as_str()).map_or(false, |v| v == b)
            }
            game::guard::ValueGuard::Int(i) => {
                self.ints.get(name.as_str()).map_or(false, |v| i.contains(v))
            }
            game::guard::ValueGuard::Num(n) => {
                self.nums.get(name.as_str()).map_or(false, |v| n.contains(v))
            }
        }
    }

    fn update_with_value(&mut self, game::trigger::StatChange { name, change, .. }: &game::trigger::StatChange<Self>) {
        match change {
            game::trigger::ValueChange::SetBool(b) => {
                self.bools.insert(name.clone(), *b);
            }
            game::trigger::ValueChange::SetInt(i) => {
                self.ints.insert(name.clone(), *i);
            }
            game::trigger::ValueChange::SetFloat(n) => {
                self.nums.insert(name.clone(), *n);
            }
            game::trigger::ValueChange::UpdateInt(ci) => {
                if let Some(oi) = self.ints.remove(name.as_str()) {
                    self.ints.insert(name.clone(), oi + ci);
                } else {
                    panic!("Missing float for name {:?}!", name.as_str());
                }
            }
            game::trigger::ValueChange::UpdateFloat(cn) => {
                if let Some(on) = self.nums.remove(name.as_str()) {
                    self.nums.insert(name.clone(), on + cn);
                } else {
                    panic!("Missing float for name {:?}!", name.as_str());
                }
            }
            game::trigger::ValueChange::Custom(cmd) => {
                match cmd.as_str() {
                    "toggleBool" => {
                        if let Some(b) = self.bools.remove(name.as_str()) {
                            self.bools.insert(name.clone(), !b);
                        } else {
                            panic!("Missing boolean value for name {:?}!", name.as_str());
                        }
                    }
                    _ => panic!("Unknown command {:?}!", cmd),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Select(usize),
    Progress,
    PromptRetry,
}

impl uial::input::CustomAction for Action {
    fn parse_input(cmd: Option<String>) -> Result<uial::input::SystemAction<Action>, String> {
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
            uial::input::SystemAction::Action(action)
        } else {
            uial::input::SystemAction::Exit
        };
        Ok(action)
    }
}

// TODO Reconsider if this is necessary at all. Mostly due to the fact that if this object is
// created, the intent is to load the scene. Instead of doing lazy loading, we can just load it
// immediately, since there's basically 100% chance we'll use it.
pub struct LoadedScene {
    name: String,
    scene: std::sync::RwLock<*const game::Scene<Store>>,
}

impl LoadedScene {
    fn new(name: String) -> Self {
        Self {
            name,
            scene: std::sync::RwLock::new(std::ptr::null()),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn load_scene(name: &str, cfg: &game::Cfg) -> Result<Box<game::Scene<Store>>, game::LoadErr> {
        use std::{fs::File, io::BufReader};

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

    fn get_or_load(&self, cfg: &game::Cfg) -> Result<(std::sync::RwLockReadGuard<*const game::Scene<Store>>, &game::Scene<Store>), exec::ResolutionErr> {
        let scene_guard = self.scene.read().expect("Not a poisoned mutex.");
        if scene_guard.is_null() {
            let scene = unsafe { &**scene_guard };
            return Ok((scene_guard, scene));
        } else {
            *self.scene.write().expect("Not a poisoned mutex.") = Box::leak(
                Self::load_scene(self.name(), cfg)
                    .map_err(|e| format!("Scene load of {} failed with {:?}", self.name(), e))?
            );
            self.get_or_load(cfg)
        }
    }
}

impl Drop for LoadedScene {
    fn drop(&mut self) {
        let guard = self.scene.write().unwrap();
        unsafe { Box::from_raw(*guard as *mut game::Scene<Store>) };
    }
}

pub enum MaybeMainOrOtherScene {
    None,
    MainScene(Option<usize>),
    String(String, Option<usize>),
}

pub struct DisplayData {
    error_text: Option<String>,
}

pub struct Sim {
    pub store: Store,
    pub scene: LoadedScene,
    pub curr_line: usize,
    pub display: uial::display::CmdDisplay<Self, game::Cfg, DisplayData>,
}

impl Sim {
    fn is_at_choice(&mut self, cfg: &<Self as exec::Sim>::Cfg) -> Result<bool, exec::ResolutionErr> {
        self.scene.get_or_load(cfg)?
            .1
            .is_line_choice(self.curr_line)
            .map_err(|_| {
                format!(
                        "Current line ({}:{}) doesn't exist???",
                        self.curr_line,
                        self.scene.name(),
                ).into()
            })
    }

    fn jump_to_scene(&mut self, scene: LoadedScene, line_num: Option<usize>) {
        self.scene = scene;
        self.curr_line = line_num.unwrap_or(0);
    }

    fn apply_trigger(store: &mut Store, game::trigger::Trigger {
        scene_change,
        stats_changes,
    }: &game::trigger::Trigger<Store>) -> Option<(LoadedScene, Option<usize>)> {
        if let Some(changes) = stats_changes {
            for change in changes {
                store.update_with_value(&change);
            }
        }
        scene_change
            .as_ref()
            .map(|new_scene| new_scene.to_inner())
            .map(|(new_scene, line)| (LoadedScene::new(new_scene), line))
    }

    fn process_choice_selection(&mut self, cfg: &<Self as exec::Sim>::Cfg, choice: usize) -> Result<(), exec::ResolutionErr> {
        let trigger = match self.scene.get_or_load(cfg)?.1.get_line_and_visible_choice(&self.store, self.curr_line, choice) {
            Ok(c) => Ok(c.trigger.as_ref()),
            Err(game::LineOrChoiceAbsenceError::LineDoesNotExist) =>
                Err(format!(
                    "Current line ({}:{}) doesn't exist???",
                    self.curr_line,
                    self.scene.name(),
                )),
            Err(game::LineOrChoiceAbsenceError::SelectionIsTriggerNotLine) =>
                Err(format!(
                    "Current line ({}:{}) is a trigger, not a line.",
                    self.curr_line,
                    self.scene.name(),
                )),
            Err(game::LineOrChoiceAbsenceError::Choice(game::AbsenceError::NotEnough)) =>
                Err(format!(
                        "Current line ({}:{}) doesn't have choices, but asked for choice {}???",
                        self.curr_line,
                        self.scene.name(),
                        choice,
                )),
            Err(game::LineOrChoiceAbsenceError::Choice(game::AbsenceError::NotPresentAfterScan)) =>
                Err(format!(
                        "Current line ({}:{}) doesn't contain choice {}???",
                        self.curr_line,
                        self.scene.name(),
                        choice,
                )),
        }?;
        if let Some(t) = trigger {
            if let Some((scene, line)) = Self::apply_trigger(&mut self.store, t) {
                self.jump_to_scene(scene, line)
            }
        }
        Ok(())
    }

    // TODO Combine with previous methods somehow.
    fn process_default_choice_selection(&mut self, cfg: &<Self as exec::Sim>::Cfg) -> Result<(), exec::ResolutionErr> {
        let trigger = match self.scene.get_or_load(cfg)?.1.get_line_and_default_choice(&self.store, self.curr_line) {
            Ok(c) => Ok(c.trigger.as_ref()),
            Err(game::LineOrChoiceAbsenceError::LineDoesNotExist) =>
                Err(format!(
                    "Current line ({}:{}) doesn't exist???",
                    self.curr_line,
                    self.scene.name(),
                )),
            Err(game::LineOrChoiceAbsenceError::SelectionIsTriggerNotLine) =>
                Err(format!(
                    "Current line ({}:{}) is a trigger, not a line.",
                    self.curr_line,
                    self.scene.name(),
                )),
            Err(game::LineOrChoiceAbsenceError::Choice(game::AbsenceError::NotEnough)) =>
                Err(format!(
                        "Current line ({}:{}) doesn't have enough choices for the default choice???",
                        self.curr_line,
                        self.scene.name(),
                )),
            Err(game::LineOrChoiceAbsenceError::Choice(game::AbsenceError::NotPresentAfterScan)) =>
                Err(format!(
                        "Current line ({}:{}) doesn't contain the default choice???",
                        self.curr_line,
                        self.scene.name(),
                )),
        }?;
        if let Some(t) = trigger {
            if let Some((scene, line)) = Self::apply_trigger(&mut self.store, t) {
                self.jump_to_scene(scene, line)
            }
        }
        Ok(())
    }

    fn progress_to_next_line_or_scene_break(&mut self, cfg: &<Self as exec::Sim>::Cfg) -> Result<usize, MaybeMainOrOtherScene> {
        let (_, curr_scene) = self.scene.get_or_load(cfg).map_err(|_| MaybeMainOrOtherScene::None)?;
        for (line, idx) in (&curr_scene.lines[self.curr_line..]).iter().zip(self.curr_line..) {
            match line.try_to_inner(&self.store) {
                Ok(game::line::LineOrTrigger::Line(_)) => {
                    return Ok(idx);
                }
                Ok(game::line::LineOrTrigger::Trigger(trigger)) => {
                    if let Some((scene, line)) = Self::apply_trigger(&mut self.store, trigger) {
                        return Err(MaybeMainOrOtherScene::String(scene.name().to_owned(), line))
                    }
                }
                Err(_) => {}
            }
        }
        // And then, if we run out of lines...
        match curr_scene.next_scene.as_ref() {
            Some(game::MainOrSceneChange::SceneChange(sc)) => {
                let (scene, line) = sc.to_inner();
                Err(MaybeMainOrOtherScene::String(scene, line))
            }
            Some(game::MainOrSceneChange::Main(line)) => {
                Err(MaybeMainOrOtherScene::MainScene(line.clone()))
            }
            None => Err(MaybeMainOrOtherScene::None)
        }
    }

    fn progress(&mut self, cfg: &<Self as exec::Sim>::Cfg) -> Result<(), exec::ResolutionErr> {
        loop {
            match self.progress_to_next_line_or_scene_break(cfg) {
                Err(MaybeMainOrOtherScene::String(name, line)) => {
                    self.jump_to_scene(LoadedScene::new(name), line)
                }
                Err(MaybeMainOrOtherScene::MainScene(line)) => {
                    self.jump_to_scene(LoadedScene::new(cfg.primary_scene.clone()), line)
                }
                Err(MaybeMainOrOtherScene::None) => {
                    return Err(format!(
                            "Trying to proceed from current line ({}:{}) leads to a soft lock. Was this intentional?",
                            self.curr_line,
                            self.scene.name(),
                    ).into());
                }
                Ok(line_num) => {
                    self.curr_line = line_num;
                    break;
                }
            }
        }
        Ok(())
    }
}

impl exec::Sim for Sim {
    type ActionEnum = Action;
    type Cfg = game::Cfg;

    type Store = Store;

    type DisplayData = DisplayData;

    fn resolve(
        &mut self,
        cfg: &Self::Cfg,
        a: Self::ActionEnum,
    ) -> std::result::Result<uial::display::RenderMode<Self::DisplayData>, exec::ResolutionErr> {
        let render_mode = match a {
            Self::ActionEnum::Select(choice) => {
                self.process_choice_selection(cfg, choice)?;
                self.progress(cfg)?;
                uial::display::RenderMode::Render(DisplayData {
                    error_text: None,
                })
            }
            Self::ActionEnum::Progress => {
                if self.is_at_choice(cfg)? {
                    self.process_default_choice_selection(cfg)?;
                }
                self.progress(cfg)?;
                uial::display::RenderMode::Render(DisplayData {
                    error_text: None,
                })
            }
            Self::ActionEnum::PromptRetry => {
                uial::display::RenderMode::Ignore
            }
        };
        Ok(render_mode)
    }
}

struct FilteredStandardLine<'a> {
    line: &'a game::line::Line<Store>,
    store: &'a <Sim as exec::Sim>::Store,
}

impl<'a> std::fmt::Display for FilteredStandardLine<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Welp")?;
        unimplemented!("Filtered line display.")
    }
}

impl std::fmt::Display for uial::display::RenderTup<&Sim, &game::Cfg, <Sim as exec::Sim>::DisplayData> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let uial::display::RenderTup(sim, cfg, data) = self;
        if let Some(text) = data.error_text.as_ref() {
            write!(f, "Error encountered: {}", text)?;
        }
        // Display the line, even if there was an error.
        if let Some(line) = sim.scene.get_or_load(cfg).unwrap().1.lines.get(sim.curr_line) {
            let line = match &line.to_inner() {
                game::line::LineOrTrigger::Line(l) => {l},
                game::line::LineOrTrigger::Trigger(_) => {
                    // TODO Announce error.
                    panic!("Current line ({}:{}) is a weird thing.", sim.curr_line, sim.scene.name());
                },
            };
            write!(
                f,
                "{}",
                FilteredStandardLine {
                    line,
                    store: &sim.store,
                }
            )
        } else {
            write!(f, "Oops, an error has occurred.")
        }
    }
}

// in, out, stable state, unstable state
pub fn run_app<Sim: exec::Sim>(
    mut input: impl uial::input::Input<Sim::ActionEnum>,
    mut display: impl uial::display::Display<Sim, Sim::Cfg, Sim::DisplayData>,
    cfg: Sim::Cfg,
    mut sim: Sim,
    mut init_disp_data: Sim::DisplayData,
) -> crate::SeRes<()> {
    // Render once to get the ball rolling.
    display.display(&sim, &cfg, init_disp_data)?;
    loop {
        let action = input.next_action()?;
        log::debug!("Executing action {:?}", action);
        match action {
            uial::input::SystemAction::Exit => {
                log::info!("System exit command received. Shutting down.");
                break;
            }
            uial::input::SystemAction::Action(a) => match sim.resolve(&cfg, a)? {
                uial::display::RenderMode::Render(data) => {
                    log::trace!("Render requested.");
                    display.display(&sim, &cfg, data)?
                },
                uial::display::RenderMode::Ignore => (),
            },
        };
    }
    Ok(())
}
