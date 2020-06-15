use crate::game;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cfg {
    pub scene_list: Vec<String>,
    pub scenes: std::path::PathBuf,
    pub saves: std::path::PathBuf,
    pub primary_scene: String,
    #[serde(skip)]
    pub root: std::path::PathBuf,
}

impl Cfg {
    pub fn load_from(p: &Path) -> Result<Cfg, game::InitErr> {
        let f = File::open(p.join("cfg.yaml"))?;
        let buf = BufReader::new(f);
        let mut cfg: Self = serde_yaml::from_reader(buf)?;
        cfg.root = p.to_owned();
        Ok(cfg)
    }
}
