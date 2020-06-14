use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Permission {
    Allow,
    Disallow,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatRequirement<Stat> {
    pub stat: Stat,
    pub permission: Permission,
    pub range: (std::ops::Bound<i64>, std::ops::Bound<i64>),
}
