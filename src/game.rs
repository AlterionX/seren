use std::io;

pub mod input;
pub mod display;

#[derive(Debug)]
pub enum Error {
    Initialization(InitErr),
    Load(LoadErr),
    Input(input::Err),
    Display(display::Err),
    Resolution(Resolution),
}
impl From<InitErr> for Error {
    fn from(e: InitErr) -> Self {
        Error::Initialization(e)
    }
}
impl From<LoadErr> for Error {
    fn from(e: LoadErr) -> Self {
        Error::Load(e)
    }
}
impl From<input::Err> for Error {
    fn from(e: input::Err) -> Self {
        Error::Input(e)
    }
}
impl From<display::Err> for Error {
    fn from(e: display::Err) -> Self {
        Error::Display(e)
    }
}
impl From<Resolution> for Error {
    fn from(e: Resolution) -> Self {
        Error::Resolution(e)
    }
}

#[derive(Debug)]
pub enum InitErr {
    IOErr(io::Error),
    ParseErr(serde_yaml::Error),
}
impl From<serde_yaml::Error> for InitErr {
    fn from(e: serde_yaml::Error) -> Self {
        InitErr::ParseErr(e)
    }
}
impl From<io::Error> for InitErr {
    fn from(e: io::Error) -> Self {
        InitErr::IOErr(e)
    }
}
impl From<LoadErr> for InitErr {
    fn from(e: LoadErr) -> Self {
        match e {
            LoadErr::IOErr(e) => InitErr::IOErr(e),
            LoadErr::ParseErr(e) => InitErr::ParseErr(e),
        }
    }
}

#[derive(Debug)]
pub enum LoadErr {
    IOErr(io::Error),
    ParseErr(serde_yaml::Error),
}
impl From<serde_yaml::Error> for LoadErr {
    fn from(e: serde_yaml::Error) -> Self {
        LoadErr::ParseErr(e)
    }
}
impl From<io::Error> for LoadErr {
    fn from(e: io::Error) -> Self {
        LoadErr::IOErr(e)
    }
}
impl From<InitErr> for LoadErr {
    fn from(e: InitErr) -> Self {
        match e {
            InitErr::IOErr(e) => LoadErr::IOErr(e),
            InitErr::ParseErr(e) => LoadErr::ParseErr(e),
        }
    }
}

#[derive(Debug)]
pub struct Resolution(String);
impl From<String> for Resolution {
    fn from(e: String) -> Self {
        Resolution(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait State {
    type ActionEnum: std::fmt::Debug;
    fn resolve(&mut self, a: Self::ActionEnum) -> Result<display::RenderMode>;
}
