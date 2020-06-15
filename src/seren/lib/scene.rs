use crate::seren::lib::stats;
use serde::{Deserialize, Serialize};

mod line;
pub use line::{Choice, FilteredStandardLine, Line, LineEnum, StandardLineEnum};
mod guard;
pub use guard::{Permission, StatRequirement};
mod change;
pub use change::{SceneChange, StatChange};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BoolOrVec<T> {
    Bool(bool),
    Vec(Vec<T>),
}

impl<T> BoolOrVec<T> {
    #[cfg(test)]
    fn false_val() -> Self {
        Self::Bool(false)
    }
    fn true_val() -> Self {
        Self::Bool(true)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene<LE: LineEnum> {
    lines: Vec<Line<LE>>,
    pub next_scene: Option<String>,
    #[serde(default = "BoolOrVec::true_val")]
    pub enable_builtins: BoolOrVec<String>,
}

impl<LE: LineEnum> Scene<LE> {
    pub fn get_line(&self, line_number: usize) -> Option<&Line<LE>> {
        self.lines.get(line_number)
    }
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

pub type StandardScene = Scene<StandardLineEnum<stats::Stat>>;

#[cfg(test)]
mod tests {
    use super::{
        stats::Stat, Choice, Line, Permission, Scene, SceneChange, StandardLineEnum, StandardScene,
        StatChange, StatRequirement,
    };
    #[test]
    fn run_serialization() {
        let data: StandardScene = Scene {
            next_scene: Some("whatev".to_string()),
            enable_builtins: super::BoolOrVec::false_val(),
            lines: vec![
                Line {
                    guards: None,
                    stat_changes: None,
                    scene_change: None,
                    line: StandardLineEnum::Choice {
                        speaker: None,
                        text: "this is the text".to_string(),
                        default_choice: None,
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
                                    target_scene: Some(
                                        "Yeah, whatev, make sure this is valid when validating a cfg"
                                            .to_string(),
                                    ),
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
                                    range: (
                                        std::ops::Bound::Included(0),
                                        std::ops::Bound::Excluded(10),
                                    ),
                                }]),
                            },
                        ],
                    },
                },
                Line {
                    guards: None,
                    stat_changes: None,
                    scene_change: None,
                    line: StandardLineEnum::Plain {
                        speaker: Some("Speaker".to_string()),
                        text: "Hello".to_string(),
                    },
                },
                Line {
                    guards: None,
                    stat_changes: None,
                    scene_change: None,
                    line: StandardLineEnum::Plain {
                        speaker: None,
                        text: "Hello 2".to_string(),
                    },
                },
            ],
        };
        println!("{}", serde_yaml::to_string(&data).unwrap());
        panic!();
    }
}
