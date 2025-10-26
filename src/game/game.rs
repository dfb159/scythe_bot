use crate::game::{board::Board, player::PlayerState};


#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub players: Vec<PlayerState>,
    pub turn: u32,
}

impl Game {
    fn new() -> Self {
        
    }
}
