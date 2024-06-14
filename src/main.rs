pub mod agent;
mod campaign;
mod game;
mod game_state;

use campaign::faction::RUSVIET;
use campaign::player_mat::INDUSTRIAL;
use campaign::Player;

use game::turn;
use game_state::resources::Resource;
use game_state::PlayerState;

use agent::random::RandomAgent;

use std::io;

const DEBUG: u32 = 1;

fn main() {
    println!("Welcome to scythe statistics!");

    let joey = Player {
        name: "Joey",
        bonus_starting_coins: 0,
        bonus_starting_power: 0,
        bonus_starting_popularity: 0,
    };

    let mut state = PlayerState::new(&joey, &RUSVIET, &INDUSTRIAL);
    let agent = RandomAgent {};

    while state.turns <= 500 && !state.has_won() {
        let (primary, secondary) = turn(&mut state, &agent);

        if DEBUG >= 1 {
            println!(
                "Turn: {} == Action: {:?} == Then: {:?}",
                state.turns, primary, secondary
            );
        }

        if DEBUG >= 2 {
            println!("{state:#?}");
            println!("");
        }

        if DEBUG >= 3 {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line!");
        }
    }

    println!(
        "Congratulations! You have won the game in {} turns! {} has scored {} coins!",
        state.turns,
        state.faction_name,
        state.total_coins()
    );
    println!("{state:?}");
}
