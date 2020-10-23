use crate::game;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cfg {
    /// List of scenes that should exist in the scenes directory.
    pub scene_list: Vec<String>,
    /// Game data files location.
    pub scenes: std::path::PathBuf,
    /// Non-game-wide save file location.
    pub saves: std::path::PathBuf,
    /// Initial scene.
    pub primary_scene: String,
    /// Location of the config file. Updated when read, not intended to be saved.
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
