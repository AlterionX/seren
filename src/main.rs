mod game;
mod seren;


pub fn run_game<State: game::State, Cfg>(
    mut state: State,
    // Normalized input device
    mut input: impl game::input::Input<State::ActionEnum>,
    // Normalized output device
    display: impl game::display::Display<State, Cfg>,
    // Cfg
    cfg: Cfg
) -> game::Result<()> {
    loop {
        let action = input.next_action()?;
        println!("Executing action {:?}", action);
        match action {
            game::input::SystemAction::Exit => break,
            game::input::SystemAction::Action(a) => state.resolve(a),
        };
    }
    Ok(())
}

fn main() -> game::Result<()> {
    let cfg = seren::parse_cfg(&std::path::Path::new("./game"))?;
    println!("Parsed game: {:?}", cfg);

    let input = game::input::cmd_line(seren::parse_input);
    let display = game::display::cmd_line();

    run_game(seren::init_state(&cfg)?, input, display, cfg)?;

    Ok(())
}
