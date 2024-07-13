pub mod agent;
mod campaign;
mod game;
mod game_state;
mod network;

use agent::{
    human::{PriorityAgent, Step},
    random::RandomAgent,
    Agent,
};
use campaign::faction::RUSVIET;
use campaign::player_mat::INDUSTRIAL;
use campaign::{Player, PrimaryAction};

use game::turnhelper::turn;
use game::turnmask::{Tile, TurnMask};
use game_state::PlayerState;

use std::io;

const DEBUG: u32 = 2;

fn main() {
    println!("Welcome to scythe statistics!");

    let player = Player {
        name: "Joey",
        bonus_starting_coins: 0,
        bonus_starting_power: 0,
        bonus_starting_popularity: 0,
    };

    let faction = RUSVIET;
    let player_mat = INDUSTRIAL;

    let mut state = PlayerState::new(&player, &faction, &player_mat);
    let mut agent = PriorityAgent {
        priority: vec![
            Step::Population,
            Step::Upgrade,
            Step::Build,
            Step::Recruit,
            Step::Power,
            Step::Popularity,
            Step::Deploy,
        ],
        final_step: PrimaryAction::Tax,
        mill_tile: Tile::Mountain,
    };

    while state.turns <= 100 {
        let action = agent.get_action(&state);
        let newstate = turn(state, &action);

        if DEBUG >= 1 {
            match action {
                TurnMask::PrimaryOnly(primary) => {
                    println!("Turn: {} == Action: {:?}", state.turns, primary)
                }
                TurnMask::PrimaryAndSecondary(primary, secondary) => {
                    println!(
                        "Turn: {} == Action: {:?} == Then: {:?}",
                        state.turns, primary, secondary
                    )
                }
            }
        }

        if DEBUG >= 2 {
            println!("{action:?}");
            println!("");
        }

        if DEBUG >= 3 {
            println!("{newstate:#?}");
            println!("");
        }

        if DEBUG >= 4 {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line!");
        }
        state = newstate;
    }

    println!(
        "Congratulations! You have won the game in {} turns! {} has scored {} coins!",
        state.turns,
        faction.name,
        state.total_coins()
    );
    println!("You have achieved the following stars:");
    println!("Upgrades: {}", state.upgrades.star);
    println!("Mechs: {}", state.mechs.star);
    println!("Buildings: {}", state.buildings.star);
    println!("Recruits: {}", state.recruits.star);
    println!("Military: {}", state.military.star);
    println!("Popularity: {}", state.popularity.star);
    println!("Total coins: {}", state.total_coins());
    println!("Total stars: {}", state.stars());

    if DEBUG >= 1 {
        println!("{state:?}");
    }
}
