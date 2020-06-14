use crate::seren::lib::stats;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    change::{SceneChange, StatChange},
    guard::StatRequirement,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice<Stat> {
    pub text: String,
    pub stat_changes: Option<Vec<StatChange<Stat>>>,
    pub scene_change: Option<SceneChange>,
    pub guards: Option<Vec<StatRequirement<Stat>>>,
}

impl<Stat> std::fmt::Display for Choice<Stat> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

pub trait LineEnum {
    type Stat: Serialize + DeserializeOwned + std::fmt::Debug;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StandardLineEnum<Stat> {
    Choice {
        text: String,
        speaker: Option<String>,
        default_choice: Option<usize>,
        choices: Vec<Choice<Stat>>,
    },
    Plain {
        text: String,
        speaker: Option<String>,
    },
    Trigger,
}

impl<'de, Stat: Serialize + DeserializeOwned + std::fmt::Debug> LineEnum
    for StandardLineEnum<Stat>
{
    type Stat = Stat;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Line<LE: LineEnum> {
    // As of now, scene_change originating from the line field overrides the local scene_change field.
    #[serde(flatten)]
    pub line: LE,
    pub guards: Option<Vec<StatRequirement<LE::Stat>>>,
    pub stat_changes: Option<Vec<StatChange<LE::Stat>>>,
    pub scene_change: Option<SceneChange>,
}

impl<Stat> std::fmt::Display for StandardLineEnum<Stat> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StandardLineEnum::Choice {
                text,
                default_choice,
                choices,
                speaker,
            } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
                for (idx, choice) in choices.iter().enumerate() {
                    if default_choice.unwrap_or(choices.len()) == idx {
                        write!(f, "\n\t{}. {} (default)", idx + 1, choice)?;
                    } else {
                        write!(f, "\n\t{}. {}", idx + 1, choice)?;
                    }
                }
            }
            StandardLineEnum::Plain { text, speaker } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
            }
            StandardLineEnum::Trigger => (),
        }
        Ok(())
    }
}

pub struct FilteredStandardLine<'a, 'b, Stat, StatStore> {
    pub line: &'a StandardLineEnum<Stat>,
    pub stats: Option<&'b StatStore>,
}

impl<'a, 'b, Stat, StatStore: stats::StatStore<Stat> + Default>
    FilteredStandardLine<'a, 'b, Stat, StatStore>
{
    pub fn get_filtered_choices(&self) -> Vec<&'a Choice<Stat>> {
        match self.line {
            StandardLineEnum::Choice { choices, .. } => choices
                .into_iter()
                .filter(|c| {
                    if let Some(guards) = c.guards.as_ref() {
                        let stats = self.stats.ok_or_else(|| StatStore::default());
                        let stats_ref = match &stats {
                            Err(e) => e,
                            Ok(a) => *a,
                        };
                        guards.iter().all(|req| stats_ref.verify(req))
                    } else {
                        true
                    }
                })
                .collect(),
            StandardLineEnum::Plain { .. } | StandardLineEnum::Trigger => vec![],
        }
    }

    pub fn get_default_choice_idx(&self) -> Option<usize> {
        match self.line {
            StandardLineEnum::Choice {
                choices,
                default_choice,
                ..
            } => {
                let default_choice = if let Some(default_choice) = default_choice {
                    *default_choice
                } else {
                    return None;
                };
                let stats = self.stats.ok_or_else(|| StatStore::default());
                let removed_choices = choices.as_slice()[0..default_choice]
                    .iter()
                    .filter(|c| {
                        if let Some(guards) = c.guards.as_ref() {
                            let stats_ref = match &stats {
                                Err(e) => e,
                                Ok(a) => *a,
                            };
                            guards.iter().all(|req| stats_ref.verify(req))
                        } else {
                            true
                        }
                    })
                    .count();
                Some(default_choice - removed_choices)
            }
            StandardLineEnum::Plain { .. } | StandardLineEnum::Trigger => None,
        }
    }
}

impl<'a, 'b, Stat, StatStore: stats::StatStore<Stat> + Default> std::fmt::Display
    for FilteredStandardLine<'a, 'b, Stat, StatStore>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.line {
            StandardLineEnum::Choice { text, speaker, .. } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
                let valid_choices = self.get_filtered_choices();
                let default_choice = self.get_default_choice_idx();
                for (idx, choice) in valid_choices.into_iter().enumerate() {
                    if default_choice.map_or(false, |default_choice| default_choice == idx) {
                        write!(f, "\n\t{}. {} (default)", idx + 1, choice)?;
                    } else {
                        write!(f, "\n\t{}. {}", idx + 1, choice)?;
                    }
                }
            }
            line_enum => write!(f, "{}", line_enum)?,
        }
        Ok(())
    }
}
