use serde::{Serialize, Deserialize};
use crate::seren::lib::stats;

#[derive(Serialize, Deserialize)]
pub struct StatChange<Stat> {
    pub stat: Stat,
    pub change: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Permission {
    Allow,
    Disallow,
}

#[derive(Serialize, Deserialize)]
pub struct StatRequirement<Stat> {
    pub stat: Stat,
    pub permission: Permission,
    pub range: (std::ops::Bound<i64>, std::ops::Bound<i64>),
}

#[derive(Serialize, Deserialize)]
pub struct SceneChange {
    pub display: String,
    pub target_scene: Option<String>,
    pub target_line: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice<Stat> {
    pub display: String,
    pub stat_changes: Option<Vec<StatChange<Stat>>>,
    pub scene_change: Option<SceneChange>,
    pub guards: Option<Vec<StatRequirement<Stat>>>,
}

impl<Stat> std::fmt::Display for Choice<Stat> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO possibly render the rest?
        write!(f, "{}", self.display)
    }
}

#[derive(Serialize, Deserialize)]
pub enum StandardLineEnum<Stat> {
    Choice {
        text: String,
        choices: Vec<Choice<Stat>>,
    },
    Plain(String),
}

impl<Stat> std::fmt::Display for StandardLineEnum<Stat> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StandardLineEnum::Choice {
                text,
                choices,
            } => {
                write!(f, "{}", text)?;
                for (idx, choice) in choices.iter().enumerate() {
                    write!(f, "\n\t{}. {}", idx + 1, choice)?;
                }
            },
            StandardLineEnum::Plain(text) => {
                write!(f, "{}", text)?;
            },
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Scene<LineEnum> {
    lines: Vec<LineEnum>,
    pub next_scene: String,
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
            next_scene: "whatev".to_string(),
            lines: vec![
                StandardLineEnum::Choice {
                    text: "this is the text".to_string(),
                    choices: vec![
                        Choice {
                            display: "choice 0".to_string(),
                            stat_changes: None,
                            scene_change: None,
                            guards: None,
                        },
                        Choice {
                            display: "choice 1".to_string(),
                            stat_changes: Some(vec![StatChange {
                                stat: Stat::Bossiness,
                                change: 0,
                            }]),
                            scene_change: None,
                            guards: None,
                        },
                        Choice {
                            display: "choice 2".to_string(),
                            stat_changes: None,
                            scene_change: Some(SceneChange {
                                display: "Why is this here?".to_string(),
                                target_scene: Some("Yeah, whatev, make sure this is valid when validating a cfg".to_string()),
                                target_line: Some(0), // implies 0 aka the first line
                            }),
                            guards: None,
                        },
                        Choice {
                            display: "choice 3".to_string(),
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
                StandardLineEnum::Plain("Hello".to_string()),
            ],
        };
        println!("{}", serde_yaml::to_string(&data).unwrap());
        // panic!();
    }
}