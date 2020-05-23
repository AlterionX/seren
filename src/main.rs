//! TODO Audio device

mod game;
mod seren;

use structopt::StructOpt;

pub fn run_app<State: game::State>(
    mut state: State,
    // Normalized input device
    mut input: impl game::input::Input<State::ActionEnum>,
    // Normalized output device
    mut display: impl game::display::Display<State, State::Cfg>,
    // Cfg
    cfg: State::Cfg
) -> game::Result<()> {
    // Render once to get the ball rolling.
    display.display(&state, &cfg)?;
    loop {
        let action = input.next_action()?;
        println!("Executing action {:?}", action);
        match action {
            game::input::SystemAction::Exit => break,
            game::input::SystemAction::Action(a) => match state.resolve(&cfg, a)? {
                game::display::RenderMode::Render => display.display(&state, &cfg)?,
                game::display::RenderMode::Ignore => (),
            },
        };
    }
    Ok(())
}

fn main() -> game::Result<()> {

    let opts = seren::CommandLineInterface::from_args();
    println!("Cmdline options: {:?}", opts);

    let cfg = seren::GameCfg::load_from(opts.game_cfg_path.as_path())?;
    println!("Game cfg loaded: {:?}", cfg);

    if opts.use_editor {
        let input = game::input::cmd_line(seren::editor::Action::parse_input);
        let display = game::display::cmd_line();
        run_app(seren::EditorState::new(cfg), input, display, seren::EditorCfg)?;
    } else {
        let input = game::input::cmd_line(seren::game::Action::parse_input);
        let display = game::display::cmd_line();
        run_app(seren::GameState::init(&cfg)?, input, display, cfg)?;
    }

    Ok(())
}
