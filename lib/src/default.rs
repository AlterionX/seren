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
pub enum ActionEnum {
    Select(usize),
    Progress,
    PromptRetry,
}

pub enum LoadedScene {
    Name(String),
    Scene(String, game::Scene<Store>),
}

impl LoadedScene {
    fn name(&self) -> &str {
        match self {
            Self::Name(n) | Self::Scene(n, _) => {
                n.as_str()
            }
        }
    }

    fn get_or_load(&mut self) -> &game::Scene<Store> {
        // TODO Implement
        unimplemented!("Scene loading not yet implemented!")
    }
}

pub enum MaybeMainOrOtherScene {
    None,
    MainScene(Option<usize>),
    String(String, Option<usize>),
}

pub struct Sim {
    pub store: Store,
    pub scene: LoadedScene,
    pub curr_line: usize,
    pub display: uial::display::CmdDisplay<Self, game::Cfg>,
}

impl Sim {
    fn is_at_choice(&mut self) -> Result<bool, exec::ResolutionErr> {
        self.scene.get_or_load()
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
            .map(|(new_scene, line)| (LoadedScene::Name(new_scene), line))
    }

    fn process_choice_selection(&mut self, choice: usize) -> Result<(), exec::ResolutionErr> {
        let trigger = match self.scene.get_or_load().get_line_and_visible_choice(&self.store, self.curr_line, choice) {
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
    fn process_default_choice_selection(&mut self) -> Result<(), exec::ResolutionErr> {
        let trigger = match self.scene.get_or_load().get_line_and_default_choice(&self.store, self.curr_line) {
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

    fn progress_to_next_line_or_scene_break(&mut self) -> Result<usize, MaybeMainOrOtherScene> {
        let curr_scene = self.scene.get_or_load();
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
            match self.progress_to_next_line_or_scene_break() {
                Err(MaybeMainOrOtherScene::String(name, line)) => {
                    self.jump_to_scene(LoadedScene::Name(name), line)
                }
                Err(MaybeMainOrOtherScene::MainScene(line)) => {
                    self.jump_to_scene(LoadedScene::Name(cfg.primary_scene.clone()), line)
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
    type ActionEnum = ActionEnum;
    type Cfg = game::Cfg;

    type Store = Store;

    fn resolve(
        &mut self,
        cfg: &Self::Cfg,
        a: Self::ActionEnum,
    ) -> std::result::Result<uial::display::RenderMode, exec::ResolutionErr> {
        let render_mode = match a {
            Self::ActionEnum::Select(choice) => {
                self.process_choice_selection(choice)?;
                self.progress(cfg)?;
                uial::display::RenderMode::Render
            }
            Self::ActionEnum::Progress => {
                if self.is_at_choice()? {
                    self.process_default_choice_selection()?;
                }
                self.progress(cfg)?;
                uial::display::RenderMode::Render
            }
            Self::ActionEnum::PromptRetry => {
                uial::display::RenderMode::Ignore
            }
        };
        Ok(render_mode)
    }
}
