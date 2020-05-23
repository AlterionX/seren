use serde::{Serialize, Deserialize};
use std::{path::Path, fs::File, io::BufReader};
use crate::game;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cfg {
    pub scene_list: Vec<String>,
    pub scenes: std::path::PathBuf,
    pub primary_scene: String,
}

impl Cfg {
    pub fn load_from(p: &Path) -> Result<Cfg, game::InitErr> {
        // TODO parse path
        let f = File::open(p.join("cfg.yaml"))?;
        let buf = BufReader::new(f);
        Ok(serde_yaml::from_reader(buf)?)
    }
}