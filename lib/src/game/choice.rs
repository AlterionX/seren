use crate::game::state::KeyedStore;
use super::{guard, trigger};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct Choice<T: KeyedStore> {
    #[serde(default)]
    pub guard: Option<guard::Guard<T>>,
    pub text: String,
    #[serde(default)]
    pub trigger: Option<trigger::Trigger<T>>,
}
