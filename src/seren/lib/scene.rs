use serde::{Serialize, Deserialize};
use tap::*;
use crate::seren::lib::stats;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct StatChange<Stat> {
    pub stat: Stat,
    pub change: i64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Permission {
    Allow,
    Disallow,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct StatRequirement<Stat> {
    pub stat: Stat,
    pub permission: Permission,
    pub range: (std::ops::Bound<i64>, std::ops::Bound<i64>),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SceneChange {
    pub target_scene: Option<String>,
    pub target_line: Option<usize>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Choice<Stat> {
    pub text: String,
    pub stat_changes: Option<Vec<StatChange<Stat>>>,
    pub scene_change: Option<SceneChange>,
    pub guards: Option<Vec<StatRequirement<Stat>>>,
}

impl<Stat> std::fmt::Display for Choice<Stat> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO possibly render the rest?
        write!(f, "{}", self.text)
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum StandardLineEnum<Stat> {
    Choice {
        text: String,
        choices: Vec<Choice<Stat>>,
        speaker: Option<String>,
    },
    Plain {
        text: String,
        speaker: Option<String>,
    },
}

impl<Stat> std::fmt::Display for StandardLineEnum<Stat> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StandardLineEnum::Choice {
                text,
                choices,
                speaker
            } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
                for (idx, choice) in choices.iter().enumerate() {
                    write!(f, "\n\t{}. {}", idx + 1, choice)?;
                }
            },
            StandardLineEnum::Plain { text, speaker } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
            },
        }
        Ok(())
    }
}

pub struct FilteredStandardLine<'a, 'b, Stat, StatStore> {
    pub line: &'a StandardLineEnum<Stat>,
    pub stats: Option<&'b StatStore>,
}

impl<'a, 'b, Stat, StatStore: stats::StatStore<Stat> + Default> FilteredStandardLine<'a, 'b, Stat, StatStore> {
    pub fn get_filtered_choices(&self) -> Vec<&'a Choice<Stat>> {
        match self.line {
            StandardLineEnum::Choice {
                choices,
                ..
            } => {
                choices
                    .into_iter()
                    .filter(|c | {
                        if let Some(guards) = c.guards.as_ref() {
                            let stats = self.stats.ok_or_else(|| StatStore::default());
                            let stats_ref = match &stats {
                                Err(e) => e,
                                Ok(a) => *a,
                            };
                            guards
                                .iter()
                                .all(|req| stats_ref.verify(req))
                        } else {
                            true
                        }
                    })
                    .collect()
            },
            StandardLineEnum::Plain { .. } => vec![],
        }
    }
}

impl<'a, 'b, Stat, StatStore: stats::StatStore<Stat> + Default> std::fmt::Display for FilteredStandardLine<'a, 'b, Stat, StatStore> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.line {
            StandardLineEnum::Choice {
                text,
                speaker,
                choices: _choices,
            } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
                let valid_choices = self.get_filtered_choices();
                for (idx, choice) in valid_choices.into_iter().enumerate() {
                    write!(f, "\n\t{}. {}", idx + 1, choice)?;
                }
            },
            StandardLineEnum::Plain {
                text,
                speaker,
            } => {
                if let Some(speaker) = speaker {
                    write!(f, "{}: ", speaker)?;
                }
                write!(f, "{}", text)?;
            },
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Scene<LineEnum> {
    lines: Vec<LineEnum>,
    pub next_scene: Option<String>,
}

impl<LE> Scene<LE> {
    pub fn get_line(&self, line_number: usize) -> Option<&LE> {
        self.lines.get(line_number)
    }
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

pub type StandardScene = Scene<StandardLineEnum<stats::Stat>>;

#[cfg(test)]
mod tests {
    use super::{Scene, StandardLineEnum, StandardScene, Choice, SceneChange, StatRequirement, StatChange, Permission, stats::Stat};
    #[test]
    fn run_serialization() {
        let data: StandardScene = Scene {
            next_scene: Some("whatev".to_string()),
            lines: vec![
                StandardLineEnum::Choice {
                    speaker: None,
                    text: "this is the text".to_string(),
                    choices: vec![
                        Choice {
                            text: "choice 0".to_string(),
                            stat_changes: None,
                            scene_change: None,
                            guards: None,
                        },
                        Choice {
                            text: "choice 1".to_string(),
                            stat_changes: Some(vec![StatChange {
                                stat: Stat::Bossiness,
                                change: 0,
                            }]),
                            scene_change: None,
                            guards: None,
                        },
                        Choice {
                            text: "choice 2".to_string(),
                            stat_changes: None,
                            scene_change: Some(SceneChange {
                                target_scene: Some("Yeah, whatev, make sure this is valid when validating a cfg".to_string()),
                                target_line: Some(0), // implies 0 aka the first line
                            }),
                            guards: None,
                        },
                        Choice {
                            text: "choice 3".to_string(),
                            stat_changes: None,
                            scene_change: None,
                            guards: Some(vec![StatRequirement {
                                stat: Stat::Bossiness,
                                permission: Permission::Allow,
                                range: (std::ops::Bound::Included(0), std::ops::Bound::Excluded(10)),
                            }]),
                        },
                    ],
                },
                StandardLineEnum::Plain {
                    speaker: Some("Speaker".to_string()),
                    text: "Hello".to_string(),
                },
                StandardLineEnum::Plain {
                    speaker: None,
                    text: "Hello 2".to_string(),
                },
            ],
        };
        println!("{}", serde_yaml::to_string(&data).unwrap());
        panic!();
    }
}