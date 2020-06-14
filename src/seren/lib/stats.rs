use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Stat {
    Bossiness,
}

impl Stat {
    pub fn default_val(&self) -> i64 {
        match self {
            Self::Bossiness => 0,
        }
    }
}

pub trait StatStore<S> {
    fn stat_value(&self, s: S) -> i64;
    fn verify(&self, req: &super::scene::StatRequirement<S>) -> bool;
    fn apply(&mut self, change: &super::scene::StatChange<S>);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    bossiness: i64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            bossiness: Stat::Bossiness.default_val(),
        }
    }
}

impl StatStore<Stat> for Stats {
    fn stat_value(&self, s: Stat) -> i64 {
        match s {
            Stat::Bossiness => self.bossiness,
        }
    }
    fn verify(&self, req: &super::scene::StatRequirement<Stat>) -> bool {
        let super::scene::StatRequirement { stat, range, .. } = req;
        match stat {
            Stat::Bossiness => {
                let val = self.bossiness;
                use std::ops::RangeBounds;
                range.contains(&val)
            }
        }
    }
    fn apply(&mut self, req: &super::scene::StatChange<Stat>) {
        let super::scene::StatChange { stat, change } = req;
        match stat {
            Stat::Bossiness => self.bossiness += change,
        };
    }
}
