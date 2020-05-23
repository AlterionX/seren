use serde::{Serialize, Deserialize};
use std::{path::Path, fs::File, io::BufReader};
use crate::game;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cfg {
    pub scene_list: Vec<String>,
    pub scenes: std::path::PathBuf,
    pub primary_scene: String,
    #[serde(skip)]
    pub root: std::path::PathBuf,
}

impl Cfg {
    pub fn load_from(p: &Path) -> Result<Cfg, game::InitErr> {
        let f = File::open(p.join("cfg.yaml"))?;
        let buf = BufReader::new(f);
        // TODO better way to do this?
        let mut cfg: Self = serde_yaml::from_reader(buf)?;
        cfg.root = p.to_owned();
        Ok(cfg)
    }
}