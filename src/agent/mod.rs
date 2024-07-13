use crate::{game::turnmask::TurnMask, game_state::PlayerState};

pub(crate) mod fcnn;
pub(crate) mod human;
pub(crate) mod random;

pub(crate) trait Agent {
    fn get_action(&mut self, state: &PlayerState) -> TurnMask;
}
