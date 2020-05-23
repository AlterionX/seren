use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Stat {
    Bossiness,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    bossiness: usize,
}