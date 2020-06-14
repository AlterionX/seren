use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatChange<Stat> {
    pub stat: Stat,
    pub change: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneChange {
    pub target_scene: Option<String>,
    pub target_line: Option<usize>,
}
