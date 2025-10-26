use crate::{
    game::{
        board::Board,
        player::{PlayerState, PlayerTemplate},
    },
    template::BoardTemplate,
};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub players: Vec<PlayerState>,
    pub turn: u32,
}

impl Game {
    pub fn new<const F: usize, const R: usize, const P: usize, const C: usize>(
        board_template: &BoardTemplate<F, R, P>,
        player_templates: [&PlayerInfo; C],
    ) -> Self {
        let board = Board::from_template(board_template);

        let mut starting_locations = Vec::with_capacity(P);
        for loc in board_template.starting_locations.iter() {
            let home = board.get_field(&loc.position);
            let start1 = board.get_field(&loc.start1);
            let start2 = board.get_field(&loc.start2);

            if let Some(h) = home {
                if let Some(s1) = start1 {
                    if let Some(s2) = start2 {
                        starting_locations.push((h, s1, s2));
                    }
                }
            }
        }

        let mut players = Vec::with_capacity(C);
        for info in player_templates {
            let loc = starting_locations.get(info.start_location_index);
            if let Some((h, s1, s2)) = loc {
                let new_player = PlayerState::new(&info.template, h, s1, s2);
                players.push(new_player);
            }
        }

        Game {
            board: board,
            players: players,
            turn: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfo<'a> {
    pub template: PlayerTemplate<'a>,
    pub start_location_index: usize,
}
