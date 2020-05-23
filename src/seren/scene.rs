use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Stat {
    Bossiness,
}
#[derive(Serialize, Deserialize)]
pub struct StatChange {
    stat: Stat,
    change: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Permission {
    Allow,
    Disallow,
}

#[derive(Serialize, Deserialize)]
pub struct StatRequirement {
    stat: Stat,
    permission: Permission,
    range: (std::ops::Bound<i64>, std::ops::Bound<i64>),
}

#[derive(Serialize, Deserialize)]
pub struct SceneChange {
    display: String,
    target_scene: Option<String>,
    target_line: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    display: String,
    stat_changes: Option<Vec<StatChange>>,
    scene_change: Option<SceneChange>,
    guards: Vec<StatRequirement>,
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO possibly render the rest?
        write!(f, "{}", self.display)
    }
}

#[derive(Serialize, Deserialize)]
pub enum StandardLineEnum {
    Choice(String, Vec<Choice>),
    Plain(String),
}

impl std::fmt::Display for StandardLineEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StandardLineEnum::Choice(text, choices) => {
                write!(f, "{}", text)?;
                for (idx, choice) in choices.iter().enumerate() {
                    write!(f, "{}. {}", idx + 1, choice)?;
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
}

impl<LE> Scene<LE> {
    pub fn get_line(&self, line_number: usize) -> &LE {
        &self.lines[line_number]
    }
}

pub type StandardScene = Scene<StandardLineEnum>;