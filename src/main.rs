// pub mod agent;
pub mod game;
pub mod network;
pub mod template;
pub mod turn;

// use agent::{
//     human::{PriorityAgent, Step},
//     random::RandomAgent,
//     Agent,
// };
use template::{faction::RUSVIET, player_mat::INDUSTRIAL, Player};

use std::io;

use crate::{
    game::{
        game::{Game, PlayerInfo},
        player::PlayerTemplate,
    },
    template::{
        board::NORMAL,
        faction::{NORDIC, POLANIA},
        player_mat::{AGRICULTURAL, PATRIOTIC},
    },
};

const DEBUG: u32 = 2;

fn main() {
    println!("Welcome to scythe statistics!");

    let player1 = PlayerInfo {
        template: PlayerTemplate {
            player: Player {
                name: "Joey",
                bonus_starting_coins: 0,
                bonus_starting_power: 0,
                bonus_starting_popularity: 0,
            },
            faction: RUSVIET,
            player_mat: INDUSTRIAL,
        },
        start_location_index: 0,
    };
    let player2 = PlayerInfo {
        template: PlayerTemplate {
            player: Player {
                name: "Alex",
                bonus_starting_coins: 0,
                bonus_starting_power: 0,
                bonus_starting_popularity: 0,
            },
            faction: POLANIA,
            player_mat: AGRICULTURAL,
        },
        start_location_index: 1,
    };
    let player3 = PlayerInfo {
        template: PlayerTemplate {
            player: Player {
                name: "Leo",
                bonus_starting_coins: 0,
                bonus_starting_power: 0,
                bonus_starting_popularity: 0,
            },
            faction: NORDIC,
            player_mat: PATRIOTIC,
        },
        start_location_index: 2,
    };

    let mut game = Game::new(&NORMAL, [&player1, &player2, &player3]);
    let x = game.players.get(1).unwrap();
    println!("{x :#?}");
    // let mut agent = PriorityAgent {
    //     priority: vec![
    //         Step::Population,
    //         Step::Upgrade,
    //         Step::Build,
    //         Step::Recruit,
    //         Step::Power,
    //         Step::Popularity,
    //         Step::Deploy,
    //     ],
    //     final_step: PrimaryAction::Tax,
    //     mill_tile: Tile::Mountain,
    // };

//     while game.turn <= 100 {
//         let action = agent.get_action(&state);
//         let newstate = turn(state, &action);

//         if DEBUG >= 1 {
//             match action {
//                 TurnMask::PrimaryOnly(primary) => {
//                     println!("Turn: {} == Action: {:?}", state.turns, primary)
//                 }
//                 TurnMask::PrimaryAndSecondary(primary, secondary) => {
//                     println!(
//                         "Turn: {} == Action: {:?} == Then: {:?}",
//                         state.turns, primary, secondary
//                     )
//                 }
//             }
//         }

//         if DEBUG >= 2 {
//             println!("{action:?}");
//             println!("");
//         }

//         if DEBUG >= 3 {
//             println!("{newstate:#?}");
//             println!("");
//         }

//         if DEBUG >= 4 {
//             let mut input = String::new();
//             io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read the line!");
//         }
//         state = newstate;
//     }

//     println!(
//         "Congratulations! You have won the game in {} turns! {} has scored {} coins!",
//         state.turns,
//         faction.name,
//         state.total_coins()
//     );
//     println!("You have achieved the following stars:");
//     println!("Upgrades: {}", state.upgrades.star);
//     println!("Mechs: {}", state.mechs.star);
//     println!("Buildings: {}", state.buildings.star);
//     println!("Recruits: {}", state.recruits.star);
//     println!("Military: {}", state.military.star);
//     println!("Popularity: {}", state.popularity.star);
//     println!("Total coins: {}", state.total_coins());
//     println!("Total stars: {}", state.stars());

//     if DEBUG >= 1 {
//         println!("{state:?}");
//     }
}
