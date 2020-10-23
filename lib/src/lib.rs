pub mod game; // Data (Scene impl)
pub mod exec; // Execute
pub mod uial; // IO

// SeRes is short for SeRen Result.
pub type SeRes<T> = std::result::Result<T, Error>;

pub mod default; // Default impl examples.

#[derive(Debug)]
pub enum Error {
    Initialization(game::InitErr),
    Load(game::LoadErr),
    Input(uial::input::Err),
    Display(uial::display::Err),
    Resolution(exec::ResolutionErr),
}
impl From<game::InitErr> for Error {
    fn from(e: game::InitErr) -> Self {
        Error::Initialization(e)
    }
}
impl From<game::LoadErr> for Error {
    fn from(e: game::LoadErr) -> Self {
        Error::Load(e)
    }
}
impl From<uial::input::Err> for Error {
    fn from(e: uial::input::Err) -> Self {
        Error::Input(e)
    }
}
impl From<uial::display::Err> for Error {
    fn from(e: uial::display::Err) -> Self {
        Error::Display(e)
    }
}
impl From<exec::ResolutionErr> for Error {
    fn from(e: exec::ResolutionErr) -> Self {
        Error::Resolution(e)
    }
}
