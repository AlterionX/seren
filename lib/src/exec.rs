use crate::{game, uial};

#[derive(Debug)]
pub struct ResolutionErr(pub String);
impl From<String> for ResolutionErr {
    fn from(e: String) -> Self {
        ResolutionErr(e)
    }
}

pub trait Sim {
    type ActionEnum: std::fmt::Debug;
    type Cfg: std::fmt::Debug;

    type Store: game::KeyedStore;

    // TODO Return a Renderer State or StateChange instead of RenderMode.
    fn resolve(
        &mut self,
        cfg: &Self::Cfg,
        a: Self::ActionEnum,
    ) -> std::result::Result<uial::display::RenderMode, ResolutionErr>;
}
