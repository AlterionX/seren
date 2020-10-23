//! TODO Audio device
use structopt::StructOpt;
use tap::*;

mod game;
mod editor;

mod logger;
mod util;
mod opts;

fn main() -> sl::SeRes<()> {
    logger::setup()
        .tap_err(|e| println!("Fern logger failed to initialize due to {:?}.", e))
        .map_err(|_| sl::exec::ResolutionErr("Fern logger failed to initialize.".to_string()))?;

    log::info!("SeRen loading cmdline options.");
    let opts = opts::CommandLineInterface::from_args();
    log::debug!("SeRen started with cmdline options {:?}.", opts);

    log::info!(
        "SeRen loading game cfg from {}.",
        opts.game_cfg_path.display()
    );
    let cfg = game::Cfg::load_from(opts.game_cfg_path.as_path())
        .tap_err(|e| log::error!("Cfg failed to load due to {:?}. Shutting down.", e))?;
    log::debug!("SeRen loaded game cfg {:?}.", cfg);

    let res = if opts.use_editor {
        log::info!("Launching SeRen in editor mode.");
        if opts.use_raw_mode {
            let input = sl::uial::input::cmd_line();
            let display = sl::uial::display::cmd_line();
            log::trace!("Input and display intialized. Running editor now.");
            sl::default::run_app(
                input,
                display,
                editor::Cfg,
                editor::State::new(cfg),
                Default::default(),
            )
            .tap_err(|e| log::error!("Editor has crashed due to {:?}.", e))
        } else {
            let input = sl::uial::input::cmd_line();
            let display = sl::uial::display::raw_cmd_line();
            log::trace!("Input and display intialized. Running editor now.");
            sl::default::run_app(
                input,
                display,
                editor::Cfg,
                editor::State::new(cfg),
                Default::default(),
            )
            .tap_err(|e| log::error!("Editor has crashed due to {:?}.", e))
        }
    } else {
        log::info!("Launching SeRen in game mode.");
        if opts.use_raw_mode {
            let input = sl::uial::input::cmd_line();
            let display = sl::uial::display::raw_cmd_line();
            log::trace!("Input and display intialized. Running game now.");
            sl::default::run_app(input, display, cfg, game::Sim::init(&cfg)?, Default::default())
                .tap_err(|e| log::error!("Game has crashed due to {:?}.", e))
        } else {
            let input = sl::uial::input::cmd_line();
            let display = sl::uial::display::cmd_line();
            log::trace!("Input and display intialized. Running game now.");
            sl::default::run_app(input, display, cfg, game::Sim::init(&cfg)?, Default::default())
                .tap_err(|e| log::error!("Game has crashed due to {:?}.", e))
        }
    };

    log::trace!("Shutdown complete. Terminating.");
    res
}
