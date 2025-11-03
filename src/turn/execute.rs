use crate::{game::game::Game, turn::mask::TurnMask};

pub fn turn(mut game: Game, mask: &TurnMask) {
    match mask {
        TurnMask::PrimaryOnly(primary) => {
            execute_primary(&mut game, primary)
        }
        TurnMask::PrimaryAndSecondary(primary, secondary) {
            execute_primary(&mut state, primary);
            execute_secondary(&mut state, secondary);
        }
        _ => {}
    }
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
    let cost = state.upgrades.get_upgrade_cost(&map_secondary(secondary));
    match secondary {
        Secondary::Upgrade(primary, secondary) if state.resources.oil >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Power) {
                state.military.add(1);
            }
            state.resources.oil -= cost;
            state.upgrades.upgrade(*primary, *secondary);
            state.coins += state.upgrades.get_upgrade_coins(&SecondaryUpgrade::Upgrade);
        }
        Secondary::Deploy(Deploy(mech)) if state.resources.metal >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Coin) {
                state.coins += 1;
            }
            state.resources.metal -= cost;
            state.mechs.deploy(*mech);
            state.coins += state.upgrades.get_upgrade_coins(&SecondaryUpgrade::Deploy);
        }
        Secondary::Build(building) if state.resources.wood >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                state.popularity.add(1);
            }
            state.resources.wood -= cost;
            state.buildings.built(*building);
            state.coins += state.upgrades.get_upgrade_coins(&SecondaryUpgrade::Build);
        }
        Secondary::Enlist(secondary, onetime) if state.resources.food >= cost => {
            if state.recruits.is_secondary_recruited(Recruit::Card) {
                state.cards += 1;
            }
            state.resources.food -= cost;
            state.recruits.recruit(*secondary, *onetime);
            state.coins += state.upgrades.get_upgrade_coins(&SecondaryUpgrade::Enlist);
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

pub fn map_primary(primary: &Primary) -> PrimaryAction {
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

pub fn map_secondary(secondary: &Secondary) -> SecondaryAction {
    match secondary {
        Secondary::Upgrade(_, _) => SecondaryAction::Upgrade,
        Secondary::Deploy(Deploy(_)) => SecondaryAction::Deploy,
        Secondary::Build(_) => SecondaryAction::Build,
        Secondary::Enlist(_, _) => SecondaryAction::Enlist,
    }
}

pub fn map_tile_resource(tile: &Tile) -> Option<Resource> {
    match tile {
        Tile::Woods => Some(Resource::Wood),
        Tile::Mountain => Some(Resource::Metal),
        Tile::Tundra => Some(Resource::Oil),
        Tile::Farm => Some(Resource::Food),
        Tile::Village => None,
    }
}
