use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "SeRen",
    about = "Command line interface dictates what mode to launch the game in."
)]
pub struct CommandLineInterface {
    #[structopt(long = "--use-editor")]
    pub use_editor: bool,
    #[structopt(long = "--tui")]
    pub use_raw_mode: bool,
    #[structopt(long = "--game-cfg-path", default_value = "./game")]
    pub game_cfg_path: std::path::PathBuf,
}
