use crate::{game_state::PlayerState, turn::turnmask::TurnMask};

pub mod fcnn;
pub mod human;
pub mod random;

pub trait Agent {
    fn get_action(&mut self, state: &PlayerState) -> TurnMask;
}
