mod lib;

pub mod game;
pub mod editor;
pub mod opts;

pub use self::game::{State as GameState, Cfg as GameCfg};
pub use self::editor::{State as EditorState, Cfg as EditorCfg};
pub use self::opts::CommandLineInterface;
