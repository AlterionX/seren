use crate::game::state::KeyedStore;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub enum ValueGuard {
    Bool(bool),
    Int((std::ops::Bound<i64>, std::ops::Bound<i64>)),
    Num((std::ops::Bound<f64>, std::ops::Bound<f64>)),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct KeyedGuard<T: KeyedStore> {
    pub name: T::KeyEnum,
    pub value: ValueGuard,
    #[serde(skip, default)]
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub enum Guard<T: KeyedStore> {
    Value(KeyedGuard<T>),

    Not(Box<Guard<T>>),
    And(Vec<Guard<T>>),
    Or(Vec<Guard<T>>),
}

