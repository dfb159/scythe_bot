use std::sync::OnceLock;

use crate::{
    campaign::{PrimaryAction, SecondaryAction},
    game_state::{production::ProductionState, PlayerState},
};

use super::turnmask::{
    Move::{Move1, Move2, Move3},
    Primary,
    Produce::{Produce1, Produce2, Produce3},
    Recruit, Resource, Secondary, SecondaryUpgrade, Tile, TurnMask,
};

fn produce_state() -> ProductionState {
    static PRODUCE_STATE: OnceLock<ProductionState> = OnceLock::new();
    *PRODUCE_STATE.get_or_init(|| {
        ProductionState::new(vec![
            Tile::Woods,
            Tile::Mountain,
            Tile::Tundra,
            Tile::Farm,
            Tile::Village,
        ])
    })
}

pub(crate) fn check_primary(state: &PlayerState, primary: &Primary) -> bool {
    match primary {
        Primary::Move(Move1((tile1, _))) => check_movement(&state.production, vec![tile1]),
        Primary::Move(Move2((tile1, _), (tile2, _))) => {
            check_movement(&state.production, vec![tile1, tile2])
        }
        Primary::Move(Move3((tile1, _), (tile2, _), (tile3, _))) => {
            state.upgrades.move_evolved
                && check_movement(&state.production, vec![tile1, tile2, tile3])
        }
        Primary::Tax => true,
        Primary::Trade(..) => state.coins >= 1,
        Primary::Promote => state.coins >= 1,
        Primary::Bolster => state.coins >= 1,
        Primary::Enforce => state.coins >= 1,
        Primary::Produce(Produce1(tile1)) => {
            state.can_produce() && check_movement(&produce_state(), vec![tile1])
        }
        Primary::Produce(Produce2(tile1, tile2)) => {
            state.can_produce() && check_movement(&produce_state(), vec![tile1, tile2])
        }
        Primary::Produce(Produce3(tile1, tile2, tile3)) => {
            state.can_produce()
                && state.upgrades.produce_evolved
                && check_movement(&produce_state(), vec![tile1, tile2, tile3])
        }
    }
}

/// Check if the the tiles are valid and do not exceed the production state
fn check_movement(state: &ProductionState, tiles: Vec<&Tile>) -> bool {
    let mut needed = ProductionState::new(vec![]);

    for tile in tiles {
        needed.add(tile, 1);
    }

    state.get(&Tile::Woods) >= needed.get(&Tile::Woods)
        && state.get(&Tile::Mountain) >= needed.get(&Tile::Mountain)
        && state.get(&Tile::Tundra) >= needed.get(&Tile::Tundra)
        && state.get(&Tile::Farm) >= needed.get(&Tile::Farm)
        && state.get(&Tile::Village) >= needed.get(&Tile::Village)
}

pub(crate) fn check_move_from(state: &PlayerState, from: &Tile) -> bool {
    state.production.get(from) >= 1
}

pub(crate) fn check_secondary_cost(state: &PlayerState, secondary: SecondaryAction) -> bool {
    (match secondary {
        SecondaryAction::Upgrade => state.resources.oil,
        SecondaryAction::Deploy => state.resources.metal,
        SecondaryAction::Build => state.resources.wood,
        SecondaryAction::Enlist => state.resources.food,
    }) >= state.upgrades.get_upgrade_cost(secondary)
}

/// Execute one turn and return the new state
pub(crate) fn turn(mut state: PlayerState, mask: &TurnMask) -> PlayerState {
    match mask {
        TurnMask::PrimaryOnly(primary) if check_primary(&state, primary) => {
            execute_primary(&mut state, primary)
        }
        TurnMask::PrimaryAndSecondary(primary, secondary) if check_primary(&state, primary) => {
            execute_primary(&mut state, primary);
            execute_secondary(&mut state, secondary);
        }
        _ => {}
    }
    state
}

fn execute_primary(mut state: &mut PlayerState, primary: &Primary) {
    state.turns += 1;
    match primary {
        Primary::Move(Move1(tiles1)) => {
            move_people(&mut state, tiles1);
        }
        Primary::Move(Move2(tiles1, tiles2)) => {
            move_people(&mut state, tiles1);
            move_people(&mut state, tiles2);
        }
        Primary::Move(Move3(tiles1, tiles2, tiles3)) => {
            move_people(&mut state, tiles1);
            move_people(&mut state, tiles2);
            move_people(&mut state, tiles3);
        }
        Primary::Tax => {
            state.coins += if state.upgrades.tax_evolved { 2 } else { 1 };
        }
        Primary::Trade(res1, res2) => {
            state.coins -= 1;
            state.resources.add(*res1, 1);
            state.resources.add(*res2, 1);
            if state.buildings.armory_built {
                state.military.add(1);
            }
        }
        Primary::Promote => {
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
        Primary::Bolster => {
            state.coins -= 1;
            let power_increase = if state.upgrades.power_evolved { 3 } else { 2 };
            state.military.add(power_increase);

            if state.buildings.monument_built {
                state.popularity.add(1);
            }
        }
        Primary::Enforce => {
            state.coins -= 1;
            let card_increase = if state.upgrades.card_evolved { 2 } else { 1 };
            state.cards += card_increase;

            if state.buildings.monument_built {
                state.popularity.add(1);
            }
        }
        Primary::Produce(Produce1(tile1)) => {
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

            produce_resource(&mut state, tile1);

            match state.buildings.mill_location {
                Some(location) => {
                    produce_resource(&mut state, &location);
                }
                None => {}
            }
        }
        Primary::Produce(Produce2(tile1, tile2)) => {
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

            produce_resource(&mut state, tile1);
            produce_resource(&mut state, tile2);

            match state.buildings.mill_location {
                Some(location) => {
                    produce_resource(&mut state, &location);
                }
                None => {}
            }
        }
        Primary::Produce(Produce3(tile1, tile2, tile3)) => {
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

            produce_resource(&mut state, tile1);
            produce_resource(&mut state, tile2);
            produce_resource(&mut state, tile3);

            match state.buildings.mill_location {
                Some(location) => {
                    produce_resource(&mut state, &location);
                }
                None => {}
            }
        }
    }
}

fn execute_secondary(state: &mut PlayerState, secondary: &Secondary) {
    let cost = state.upgrades.get_upgrade_cost(map_secondary(secondary));
    match secondary {
        Secondary::Upgrade(primary, secondary) if state.resources.oil >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Power) {
                state.military.add(1);
            }
            state.resources.oil -= cost;
            state.upgrades.upgrade(*primary, *secondary);
            state.coins += state.upgrades.get_upgrade_coins(SecondaryUpgrade::Upgrade);
        }
        Secondary::Deploy(mech) if state.resources.metal >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Coin) {
                state.coins += 1;
            }
            state.resources.metal -= cost;
            state.mechs.deploy(*mech);
            state.coins += state.upgrades.get_upgrade_coins(SecondaryUpgrade::Deploy);
        }
        Secondary::Build(building) if state.resources.wood >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                state.popularity.add(1);
            }
            state.resources.wood -= cost;
            state.buildings.built(*building);
            state.coins += state.upgrades.get_upgrade_coins(SecondaryUpgrade::Build);
        }
        Secondary::Enlist(secondary, onetime) if state.resources.food >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Card) {
                state.cards += 1;
            }
            state.resources.food -= cost;
            state.recruits.recruit(*secondary, *onetime);
            state.coins += state.upgrades.get_upgrade_coins(SecondaryUpgrade::Enlist);
        }
        _ => {}
    }
}

fn move_people(state: &mut PlayerState, (from, to): &(Tile, Tile)) {
    if check_move_from(state, from) {
        state.production.add(from, -1);
        state.production.add(to, 1);
    }
}

fn produce_resource(state: &mut PlayerState, tile: &Tile) {
    match tile {
        Tile::Village => {
            state
                .production
                .add(&Tile::Village, state.production.get(&Tile::Village));
        }
        _ => match map_tile_resource(&tile) {
            Some(resource) => {
                state.resources.add(resource, state.production.get(tile));
            }
            None => {}
        },
    }
}

pub(crate) fn map_primary(primary: &Primary) -> PrimaryAction {
    match primary {
        Primary::Move(_) => PrimaryAction::Move,
        Primary::Tax => PrimaryAction::Tax,
        Primary::Trade(_, _) => PrimaryAction::Trade,
        Primary::Promote => PrimaryAction::Promote,
        Primary::Bolster => PrimaryAction::Bolster,
        Primary::Enforce => PrimaryAction::Enforce,
        Primary::Produce(_) => PrimaryAction::Produce,
    }
}

pub(crate) fn map_secondary(secondary: &Secondary) -> SecondaryAction {
    match secondary {
        Secondary::Upgrade(_, _) => SecondaryAction::Upgrade,
        Secondary::Deploy(_) => SecondaryAction::Deploy,
        Secondary::Build(_) => SecondaryAction::Build,
        Secondary::Enlist(_, _) => SecondaryAction::Enlist,
    }
}

pub(crate) fn map_tile_resource(tile: &Tile) -> Option<Resource> {
    match tile {
        Tile::Woods => Some(Resource::Wood),
        Tile::Mountain => Some(Resource::Metal),
        Tile::Tundra => Some(Resource::Oil),
        Tile::Farm => Some(Resource::Food),
        Tile::Village => None,
    }
}
