use crate::game::state::KeyedStore;
use super::{guard, trigger, choice, AbsenceError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct Choices<T: KeyedStore> {
    pub choices: Vec<choice::Choice<T>>,
    pub default_choice: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct Line<T: KeyedStore> {
    #[serde(default)]
    pub speaker: Option<String>,
    pub text: String,
    #[serde(flatten)]
    pub choices: Option<Choices<T>>,
}

impl<T: KeyedStore> Line<T> {
    pub fn try_get_visible_choice(&self, store: &T, choice: usize) -> Result<&choice::Choice<T>, AbsenceError> {
        let cc = if let Some(cc) = self.choices.as_ref() {
            &cc.choices
        } else {
            return Err(AbsenceError::NotEnough);
        };
        if cc.len() < choice {
            return Err(AbsenceError::NotEnough);
        }
        let mut curr_choice = 0;
        let mut selected_choice = None;
        for c in cc {
            if curr_choice == choice {
                selected_choice = Some(c);
                break;
            }
            if let Some(g) = c.guard.as_ref() {
                if store.check_guard(g) {
                    curr_choice += 1;
                }
            } else {
                curr_choice += 1;
            }
        }
        selected_choice.ok_or(AbsenceError::NotPresentAfterScan)
    }

    pub fn try_get_default_choice(&self, store: &T) -> Result<&choice::Choice<T>, AbsenceError> {
        let (cc, default_choice) = if let Some(cc) = self.choices.as_ref() {
            (&cc.choices, cc.default_choice)
        } else {
            return Err(AbsenceError::NotEnough);
        };
        if cc.len() < default_choice {
            return Err(AbsenceError::NotEnough);
        }
        cc
            .get(default_choice)
            .and_then(|c| if c.guard.as_ref().map(|g| store.check_guard(g)).unwrap_or(true) {
                Some(c)
            } else {
                None
            })
            .ok_or(AbsenceError::NotPresentAfterScan)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "", untagged)]
pub enum LineOrTrigger<T: KeyedStore> {
    Line(Line<T>),
    Trigger(trigger::Trigger<T>),
}

impl<T: KeyedStore> LineOrTrigger<T> {
    pub fn line(&self) -> Option<&Line<T>> {
        if let Self::Line(l) = self {
            Some(l)
        } else {
            None
        }
    }
    pub fn trigger(&self) -> Option<&trigger::Trigger<T>> {
        if let Self::Trigger(l) = self {
            Some(l)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct GuardedLineOrTrigger<T: KeyedStore> {
    #[serde(default)]
    guard: Option<guard::Guard<T>>,
    guarded: LineOrTrigger<T>,
}

impl<T: KeyedStore> GuardedLineOrTrigger<T> {
    pub fn try_to_inner(&self, store: &T) -> Result<&LineOrTrigger<T>, ()> {
        if self.guard.as_ref().map_or(false, |g| store.check_guard(g)) {
            Ok(&self.guarded)
        } else {
            Err(())
        }
    }
    // Should only be called if you're absolutely positive that it's okay.
    pub fn to_inner(&self) -> &LineOrTrigger<T> {
        &self.guarded
    }
}
