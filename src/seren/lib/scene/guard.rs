use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Permission {
    Allow,
    Disallow,
}

impl Permission {
    fn allow() -> Self {
        Permission::Allow
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatRequirement<Stat> {
    pub stat: Stat,
    #[serde(default = "Permission::allow")]
    pub permission: Permission,
    pub range: (std::ops::Bound<i64>, std::ops::Bound<i64>),
}
