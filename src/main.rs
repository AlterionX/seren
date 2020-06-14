//! TODO Audio device
use structopt::StructOpt;
use tap::*;

mod game;
mod seren;

mod logger;
mod util;

fn run_app<State: game::State>(
    mut state: State,
    // Normalized input device
    mut input: impl game::input::Input<State::ActionEnum>,
    // Normalized output device
    mut display: impl game::display::Display<State, State::Cfg>,
    // Cfg
    cfg: State::Cfg,
) -> game::Result<()> {
    // Render once to get the ball rolling.
    display.display(&state, &cfg)?;
    loop {
        let action = input.next_action()?;
        log::debug!("Executing action {:?}", action);
        match action {
            game::input::SystemAction::Exit => {
                log::info!("System exit command received. Shutting down.");
                break;
            }
            game::input::SystemAction::Action(a) => match state.resolve(&cfg, a)? {
                game::display::RenderMode::Render => display.display(&state, &cfg)?,
                game::display::RenderMode::Ignore => (),
            },
        };
    }
    Ok(())
}

fn main() -> game::Result<()> {
    logger::setup()
        .tap_err(|e| println!("Fern logger failed to initialize due to {:?}.", e))
        .map_err(|_| game::Resolution("Fern logger failed to initialize.".to_string()))?;

    log::info!("SeRen loading cmdline options.");
    let opts = seren::CommandLineInterface::from_args();
    log::debug!("SeRen started with cmdline options {:?}.", opts);

    log::info!(
        "SeRen loading game cfg from {}.",
        opts.game_cfg_path.display()
    );
    let cfg = seren::GameCfg::load_from(opts.game_cfg_path.as_path())
        .tap_err(|e| log::error!("Cfg failed to load due to {:?}. Shutting down.", e))?;
    log::debug!("SeRen loaded game cfg {:?}.", cfg);

    let res = if opts.use_editor {
        log::info!("Launching SeRen in editor mode.");
        if opts.use_raw_mode {
            let input = game::input::cmd_line(seren::editor::Action::parse_input);
            let display = game::display::cmd_line();
            log::trace!("Input and display intialized. Running editor now.");
            run_app(
                seren::EditorState::new(cfg),
                input,
                display,
                seren::EditorCfg,
            )
            .tap_err(|e| log::error!("Editor has crashed due to {:?}.", e))
        } else {
            let input = game::input::cmd_line(seren::editor::Action::parse_input);
            let display = game::display::raw_cmd_line();
            log::trace!("Input and display intialized. Running editor now.");
            run_app(
                seren::EditorState::new(cfg),
                input,
                display,
                seren::EditorCfg,
            )
            .tap_err(|e| log::error!("Editor has crashed due to {:?}.", e))
        }
    } else {
        log::info!("Launching SeRen in game mode.");
        if opts.use_raw_mode {
            let input = game::input::cmd_line(seren::game::Action::parse_input);
            let display = game::display::raw_cmd_line();
            log::trace!("Input and display intialized. Running game now.");
            run_app(seren::GameState::init(&cfg)?, input, display, cfg)
                .tap_err(|e| log::error!("Game has crashed due to {:?}.", e))
        } else {
            let input = game::input::cmd_line(seren::game::Action::parse_input);
            let display = game::display::cmd_line();
            log::trace!("Input and display intialized. Running game now.");
            run_app(seren::GameState::init(&cfg)?, input, display, cfg)
                .tap_err(|e| log::error!("Game has crashed due to {:?}.", e))
        }
    };

    log::trace!("Shutdown complete. Terminating.");
    res
}
