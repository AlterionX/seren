use crate::game::state::KeyedStore;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ValueChange {
    SetBool(bool),
    SetInt(i64),
    SetFloat(f64),
    UpdateInt(i64),
    UpdateFloat(f64),
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatChange<T: KeyedStore> {
    pub name: T::KeyEnum,
    pub change: ValueChange,
    #[serde(skip, default)]
    _phantom: std::marker::PhantomData<T>,
}

// TODO Consider dynamic new scenes.
#[derive(Serialize, Deserialize, Debug)]
pub struct SceneChange {
    name: String,
    #[serde(default)]
    target_line: Option<usize>
}

impl SceneChange {
    pub fn to_inner(&self) -> (String, Option<usize>) {
        (self.name.to_owned(), self.target_line.clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct Trigger<T: KeyedStore> {
    #[serde(default)]
    pub stats_changes: Option<Vec<StatChange<T>>>,
    #[serde(default)]
    pub scene_change: Option<SceneChange>,
}

