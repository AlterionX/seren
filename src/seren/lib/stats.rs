use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Stat {
    Bossiness,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    bossiness: i64,
}

impl Stats {
    pub fn verify(&self, req: &super::scene::StatRequirement<Stat>) -> bool {
        // TODO verify
        let super::scene::StatRequirement {
            stat,
            range,
            ..
        } = req;
        false
    }
    pub fn apply(&mut self, req: &super::scene::StatChange<Stat>) {
        let super::scene::StatChange {
            stat,
            change,
        } = req;
        match stat {
            Stat::Bossiness => {
                self.bossiness += change
            },
        };
    }
}