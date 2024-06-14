mod campaign;
mod game_state;

use campaign::faction::RUSVIET;
use campaign::player_mat::INDUSTRIAL;
use campaign::{Player, PrimaryAction, SecondaryAction};

use game_state::buildings::Building;
use game_state::mechs::Mech;
use game_state::recruits::Recruit;
use game_state::resources::Resource;
use game_state::upgrades::Upgrade;
use game_state::PlayerState;

use rand::{seq::IteratorRandom, Rng};
use std::{cmp::min, io};

trait Agent {
    fn prepare(&self, state: &PlayerState);
    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction;
    fn choose_trade(&self, state: &PlayerState) -> Resource;
    fn choose_produce(&self, state: &PlayerState) -> Resource;
    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)>;
    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)>;
    fn deploy(&self, state: &PlayerState) -> Option<Mech>;
    fn build(&self, state: &PlayerState) -> Option<Building>;
    fn choose_mill_location(&self, state: &PlayerState) -> Resource;
    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)>;
}

struct RandomAgent {}

impl Agent for RandomAgent {
    fn prepare(&self, _state: &PlayerState) {}

    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction {
        let mut choice = Vec::with_capacity(7);
        if state.can_produce() {
            choice.push(PrimaryAction::Produce);
        }
        if state.coins >= 1 {
            choice.push(PrimaryAction::Trade);
            choice.push(PrimaryAction::Promote);
            choice.push(PrimaryAction::Bolster);
            choice.push(PrimaryAction::Enforce);
        }
        choice.push(PrimaryAction::Move);
        choice.push(PrimaryAction::Tax);

        choice.into_iter().choose(&mut rand::thread_rng()).unwrap()
    }

    fn choose_trade(&self, _state: &PlayerState) -> Resource {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            _ => Resource::Food,
        }
    }

    fn choose_produce(&self, _state: &PlayerState) -> Resource {
        match rand::thread_rng().gen_range(0..=4) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            3 => Resource::Food,
            _ => Resource::People,
        }
    }

    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)> {
        let from = vec![
            Resource::Wood,
            Resource::Metal,
            Resource::Oil,
            Resource::Food,
            Resource::People,
        ]
        .into_iter()
        .filter(|resource| state.production.get(*resource) > 0)
        .choose(&mut rand::thread_rng());

        let to = match rand::thread_rng().gen_range(0..=4) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            3 => Resource::Food,
            _ => Resource::People,
        };

        match from {
            Some(from) if from != to => Some((from, to)),
            _ => None,
        }
    }

    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)> {
        let primary_list = vec![
            Upgrade::Popularity,
            Upgrade::Power,
            Upgrade::Card,
            Upgrade::Move,
            Upgrade::Tax,
            Upgrade::Produce,
        ]
        .into_iter()
        .filter(|upgrade| state.upgrades.can_upgrade_primary(*upgrade))
        .choose(&mut rand::thread_rng());

        let secondary_list = vec![
            SecondaryAction::Upgrade,
            SecondaryAction::Deploy,
            SecondaryAction::Build,
            SecondaryAction::Enlist,
        ]
        .into_iter()
        .filter(|secondary| state.upgrades.can_upgrade_secondary(*secondary))
        .choose(&mut rand::thread_rng());

        match (primary_list, secondary_list) {
            (Some(primary), Some(secondary)) => Some((primary, secondary)),
            _ => None,
        }
    }

    fn deploy(&self, state: &PlayerState) -> Option<Mech> {
        let mech_list = vec![Mech::First, Mech::Second, Mech::Third, Mech::Fourth]
            .into_iter()
            .filter(|mech| state.mechs.can_deploy(*mech))
            .choose(&mut rand::thread_rng());

        match mech_list {
            Some(mech) => Some(mech),
            _ => None,
        }
    }

    fn build(&self, state: &PlayerState) -> Option<Building> {
        let building_list = vec![
            Building::Mine,
            Building::Mill,
            Building::Armory,
            Building::Monument,
        ]
        .into_iter()
        .filter(|building| state.buildings.can_build(*building))
        .choose(&mut rand::thread_rng());

        match building_list {
            Some(building) => Some(building),
            _ => None,
        }
    }

    fn choose_mill_location(&self, _state: &PlayerState) -> Resource {
        match rand::thread_rng().gen_range(0..=4) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            3 => Resource::Food,
            _ => Resource::People,
        }
    }

    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)> {
        let secondary_list = vec![
            Recruit::Military,
            Recruit::Coin,
            Recruit::Popularity,
            Recruit::Card,
        ]
        .into_iter()
        .filter(|recruit| !state.recruits.is_secondary_recruited(*recruit))
        .choose(&mut rand::thread_rng());

        let onetime_list = vec![
            Recruit::Military,
            Recruit::Coin,
            Recruit::Popularity,
            Recruit::Card,
        ]
        .into_iter()
        .filter(|recruit| !state.recruits.is_onetime_recruited(*recruit))
        .choose(&mut rand::thread_rng());

        match (secondary_list, onetime_list) {
            (Some(secondary), Some(onetime)) => Some((secondary, onetime)),
            _ => None,
        }
    }
}

fn main() {
    println!("Welcome to scythe statistics!");

    let joey = Player {
        name: "Joey".to_string(),
        bonus_starting_coins: 0,
        bonus_starting_power: 0,
        bonus_starting_popularity: 0,
    };

    let mut state = PlayerState::new(&joey, &RUSVIET, &INDUSTRIAL);
    let agent = RandomAgent {};

    while state.turns <= 500 && !state.has_won() {
        turn(&mut state, &agent);
    }

    println!(
        "Congratulations! You have won the game in {} turns! {} has scored {} coins!",
        state.turns,
        state.faction_name,
        state.total_coins()
    );
    println!("{state:?}");
}

fn turn(state: &mut PlayerState, agent: &impl Agent) {
    agent.prepare(state);

    let choice = agent.choose_primary(state);
    execute_primary(state, agent, choice);

    state.turns += 1;

    let secondary = state.get_secondary(choice);
    execute_secondary(state, agent, secondary);

    println!("Turn: {} == Action: {:?}", state.turns, choice);
    //println!("{state:#?}");
    //println!("Turn: {} == Action: {:?}", state.turns, choice);
    //println!();
    //
    //let mut input = String::new();
    //io::stdin()
    //    .read_line(&mut input)
    //    .expect("Failed to read the line!");
}

fn execute_primary(state: &mut PlayerState, agent: &impl Agent, primary: PrimaryAction) {
    match primary {
        PrimaryAction::Move => {
            move_people(state, agent.choose_move(state));
            move_people(state, agent.choose_move(state));
            if state.upgrades.move_evolved {
                move_people(state, agent.choose_move(state));
            }
        }
        PrimaryAction::Tax => {
            state.coins += if state.upgrades.tax_evolved { 2 } else { 1 };
        }
        PrimaryAction::Trade if state.coins >= 1 => {
            state.coins -= 1;
            state.resources.add(agent.choose_trade(state), 1);
            state.resources.add(agent.choose_trade(state), 1);
            if state.buildings.armory_build {
                state.military.add(1);
            }
        }
        PrimaryAction::Promote if state.coins >= 1 => {
            state.coins -= 1;
            let popularity_increase = if state.upgrades.popularity_evolved {
                2
            } else {
                1
            };
            state.popularity.add(popularity_increase);

            if state.buildings.armory_build {
                state.military.add(1);
            }
        }
        PrimaryAction::Bolster if state.coins >= 1 => {
            state.coins -= 1;
            let power_increase = if state.upgrades.power_evolved { 3 } else { 2 };
            state.military.add(power_increase);

            if state.buildings.monument_build {
                state.popularity.add(1);
            }
        }
        PrimaryAction::Enforce if state.coins >= 1 => {
            state.coins -= 1;
            let card_increase = if state.upgrades.card_evolved { 2 } else { 1 };
            state.cards += card_increase;

            if state.buildings.monument_build {
                state.popularity.add(1);
            }
        }
        PrimaryAction::Produce if state.can_produce() => {
            // move check GameState Helper class
            let total = state.production.total();
            if total >= 4 {
                state.military.add(-1)
            }
            if total >= 6 {
                state.popularity.add(-1)
            }
            if total >= 8 {
                state.coins -= 1
            }

            produce_resource(state, agent.choose_produce(state));
            produce_resource(state, agent.choose_produce(state));

            if state.upgrades.produce_evolved {
                produce_resource(state, agent.choose_produce(state));
            }

            match state.buildings.mill_location {
                Some(location) => {
                    produce_resource(state, location);
                }
                None => {}
            }
        }
        _ => {
            panic!("Action was invalid, not enough ressources for it!")
        } // action invalid
    }
}

fn move_people(state: &mut PlayerState, dest: Option<(Resource, Resource)>) {
    match dest {
        Some((from, to)) if state.production.get(from) > 0 => {
            state.production.add(from, -1);
            state.production.add(to, 1);
        }
        _ => {}
    }
}

fn produce_resource(state: &mut PlayerState, resource: Resource) {
    match resource {
        Resource::People => {
            state
                .production
                .add(Resource::People, state.production.population);
        }
        _ => {
            state
                .resources
                .add(resource, state.production.get(resource));
        }
    }
}

fn execute_secondary(state: &mut PlayerState, agent: &impl Agent, secondary: SecondaryAction) {
    let cost = state.upgrades.get_upgrade_cost(secondary);
    match secondary {
        SecondaryAction::Upgrade if state.resources.oil >= cost => match agent.upgrade(state) {
            Some((primary, secondary)) => {
                if state.recruits.is_secondary_recruited(Recruit::Military) {
                    state.military.add(1);
                }
                state.resources.oil -= cost;
                state.upgrades.upgrade(primary, secondary);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Upgrade);
            }
            _ => {}
        },
        SecondaryAction::Deploy if state.resources.metal >= cost => match agent.deploy(state) {
            Some(mech) => {
                if state.recruits.is_secondary_recruited(Recruit::Coin) {
                    state.coins += 1;
                }
                state.resources.metal -= cost;
                state.mechs.deploy(mech);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Deploy);
            }
            _ => {}
        },
        SecondaryAction::Build if state.resources.wood >= cost => match agent.build(state) {
            Some(Building::Mill) => {
                if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                    state.popularity.add(1);
                }
                state.resources.wood -= cost;
                state.buildings.build(Building::Mill);
                state.buildings.mill_location = Some(agent.choose_mill_location(state));
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Build);
            }
            Some(building) => {
                if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                    state.popularity.add(1);
                }
                state.resources.wood -= cost;
                state.buildings.build(building);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Build);
            }
            _ => {}
        },
        SecondaryAction::Enlist if state.resources.food >= cost => match agent.enlist(state) {
            Some((secondary, onetime)) => {
                if state.recruits.is_secondary_recruited(Recruit::Card) {
                    state.cards += 1;
                }
                state.resources.food -= cost;
                state.recruits.recruit(secondary, onetime);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Enlist);
            }
            _ => {}
        },
        _ => {}
    }
}
