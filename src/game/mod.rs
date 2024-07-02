use crate::{
    agent::Agent,
    campaign::{PrimaryAction, SecondaryAction},
    game_state::{buildings::Building, recruits::Recruit, resources::Resource, PlayerState},
};

pub(crate) fn turn(
    state: &mut PlayerState,
    agent: &mut impl Agent,
) -> (PrimaryAction, SecondaryAction) {
    agent.prepare(state);

    let choice = agent.choose_primary(state);
    execute_primary(state, agent, choice);

    state.turns += 1;

    let secondary = state.get_secondary(choice);
    execute_secondary(state, agent, secondary);

    (choice, secondary)
}

fn execute_primary(state: &mut PlayerState, agent: &mut impl Agent, primary: PrimaryAction) {
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
            if state.buildings.armory_built {
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

            if state.buildings.armory_built {
                state.military.add(1);
            }
        }
        PrimaryAction::Bolster if state.coins >= 1 => {
            state.coins -= 1;
            let power_increase = if state.upgrades.power_evolved { 3 } else { 2 };
            state.military.add(power_increase);

            if state.buildings.monument_built {
                state.popularity.add(1);
            }
        }
        PrimaryAction::Enforce if state.coins >= 1 => {
            state.coins -= 1;
            let card_increase = if state.upgrades.card_evolved { 2 } else { 1 };
            state.cards += card_increase;

            if state.buildings.monument_built {
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

fn execute_secondary(state: &mut PlayerState, agent: &mut impl Agent, secondary: SecondaryAction) {
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
                state.buildings.built(Building::Mill);
                state.buildings.mill_location = Some(agent.choose_mill_location(state));
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Build);
            }
            Some(building) => {
                if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                    state.popularity.add(1);
                }
                state.resources.wood -= cost;
                state.buildings.built(building);
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
