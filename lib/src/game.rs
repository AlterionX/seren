use std::io;
use serde::{Deserialize, Serialize};

pub mod guard;
pub mod trigger;
pub mod choice;
pub mod line;

mod state;
pub use state::KeyedStore;

mod cfg;
pub use cfg::Cfg;

#[derive(Debug)]
pub enum InitErr {
    IOErr(io::Error),
    ParseErr(serde_yaml::Error),
}
impl From<serde_yaml::Error> for InitErr {
    fn from(e: serde_yaml::Error) -> Self {
        InitErr::ParseErr(e)
    }
}
impl From<io::Error> for InitErr {
    fn from(e: io::Error) -> Self {
        InitErr::IOErr(e)
    }
}
impl From<LoadErr> for InitErr {
    fn from(e: LoadErr) -> Self {
        match e {
            LoadErr::IOErr(e) => InitErr::IOErr(e),
            LoadErr::ParseErr(e) => InitErr::ParseErr(e),
        }
    }
}

#[derive(Debug)]
pub enum LoadErr {
    IOErr(io::Error),
    ParseErr(serde_yaml::Error),
}
impl From<serde_yaml::Error> for LoadErr {
    fn from(e: serde_yaml::Error) -> Self {
        LoadErr::ParseErr(e)
    }
}
impl From<io::Error> for LoadErr {
    fn from(e: io::Error) -> Self {
        LoadErr::IOErr(e)
    }
}
impl From<InitErr> for LoadErr {
    fn from(e: InitErr) -> Self {
        match e {
            InitErr::IOErr(e) => LoadErr::IOErr(e),
            InitErr::ParseErr(e) => LoadErr::ParseErr(e),
        }
    }
}

pub enum AbsenceError {
    NotEnough,
    NotPresentAfterScan,
}

pub enum LineOrChoiceAbsenceError {
    LineDoesNotExist,
    SelectionIsTriggerNotLine,
    Choice(AbsenceError),
}
impl From<AbsenceError> for LineOrChoiceAbsenceError {
    fn from(e: AbsenceError) -> Self {
        Self::Choice(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MainOrSceneChange {
    Main(Option<usize>),
    SceneChange(trigger::SceneChange),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct Scene<T: state::KeyedStore> {
    pub lines: Vec<line::GuardedLineOrTrigger<T>>,
    #[serde(default)]
    pub next_scene: Option<MainOrSceneChange>,
}

impl<T: state::KeyedStore> Scene<T> {
    pub fn get_line_and_visible_choice(&self, store: &T, line: usize, choice: usize) -> Result<&choice::Choice<T>, LineOrChoiceAbsenceError> {
        let guarded_line = self.lines.get(line).ok_or(LineOrChoiceAbsenceError::LineDoesNotExist)?;
        let line = guarded_line.to_inner().line().ok_or(LineOrChoiceAbsenceError::SelectionIsTriggerNotLine)?;
        let choice = line.try_get_visible_choice(store, choice)?;
        Ok(choice)
    }

    pub fn get_line_and_default_choice(&self, store: &T, line: usize) -> Result<&choice::Choice<T>, LineOrChoiceAbsenceError> {
        let guarded_line = self.lines.get(line).ok_or(LineOrChoiceAbsenceError::LineDoesNotExist)?;
        let line = guarded_line.to_inner().line().ok_or(LineOrChoiceAbsenceError::SelectionIsTriggerNotLine)?;
        let choice = line.try_get_default_choice(store)?;
        Ok(choice)
    }

    pub fn is_line_choice(&self, line: usize) -> Result<bool, ()> {
        self.lines.get(line)
            .and_then(|l| l.to_inner().line())
            .map(|l| l.choices.is_some())
            .ok_or(())
    }
}
