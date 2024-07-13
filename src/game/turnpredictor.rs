use crate::{
    campaign::SecondaryAction,
    game::{
        turnhelper::{check_secondary_cost, map_primary, turn},
        turnmask::{
            Building, Mech, Move, Primary, Recruit, Resource, Secondary, SecondaryUpgrade, Tile,
            TurnMask,
        },
    },
    game_state::PlayerState,
};

use super::turnmask::{PrimaryUpgrade, Produce};

pub(crate) fn get_actions(state: &PlayerState) -> Vec<TurnMask> {
    let mut actions = Vec::new();
    for primary in get_primaries(state) {
        let primary_only = TurnMask::PrimaryOnly(primary);
        actions.push(TurnMask::PrimaryOnly(primary));
        let secondary = state.get_secondary(map_primary(&primary));
        let new_state = turn(*state, &primary_only);
        if check_secondary_cost(&new_state, &secondary) {
            for secondary in get_secondaries(&new_state, secondary) {
                actions.push(TurnMask::PrimaryAndSecondary(primary, secondary));
            }
        }
    }

    actions
}

fn get_primaries(state: &PlayerState) -> Vec<Primary> {
    let mut primaries = Vec::new();

    primaries.push(Primary::Tax);
    if state.coins >= 1 {
        primaries.extend(get_trades());
        primaries.push(Primary::Promote);
        primaries.push(Primary::Bolster);
        primaries.push(Primary::Enforce);
    }
    if state.can_produce() {
        primaries.extend(get_produces(state));
    }
    primaries.extend(get_moves(state));

    primaries
}

fn get_secondaries(state: &PlayerState, secondary: SecondaryAction) -> Vec<Secondary> {
    match secondary {
        SecondaryAction::Upgrade => get_upgrades(state),
        SecondaryAction::Deploy => get_deploys(state),
        SecondaryAction::Build => get_builds(state),
        SecondaryAction::Enlist => get_enlists(state),
    }
}

fn get_enlists(state: &PlayerState) -> Vec<Secondary> {
    let mut secondary = Vec::new();
    if !state.recruits.secondary_card_recruited {
        secondary.push(Recruit::Card);
    }
    if !state.recruits.secondary_coin_recruited {
        secondary.push(Recruit::Coin);
    }
    if !state.recruits.secondary_military_recruited {
        secondary.push(Recruit::Power);
    }
    if !state.recruits.secondary_popularity_recruited {
        secondary.push(Recruit::Popularity);
    }

    let mut onetime = Vec::new();
    if !state.recruits.onetime_card_recruited {
        onetime.push(Recruit::Card);
    }
    if !state.recruits.onetime_coin_recruited {
        onetime.push(Recruit::Coin);
    }
    if !state.recruits.onetime_military_recruited {
        onetime.push(Recruit::Power);
    }
    if !state.recruits.onetime_popularity_recruited {
        onetime.push(Recruit::Popularity);
    }

    let mut recruits = Vec::new();
    for sec in secondary {
        for once in onetime.clone() {
            recruits.push(Secondary::Enlist(sec, once));
        }
    }

    recruits
}

fn get_builds(state: &PlayerState) -> Vec<Secondary> {
    let mut builds = Vec::new();
    if !state.buildings.mine_built {
        builds.push(Secondary::Build(Building::Tunnel));
    }
    if !state.buildings.armory_built {
        builds.push(Secondary::Build(Building::Armory));
    }
    if !state.buildings.monument_built {
        builds.push(Secondary::Build(Building::Monument));
    }
    if !state.buildings.mill_built {
        builds.push(Secondary::Build(Building::Mill(Tile::Woods)));
        builds.push(Secondary::Build(Building::Mill(Tile::Mountain)));
        builds.push(Secondary::Build(Building::Mill(Tile::Tundra)));
        builds.push(Secondary::Build(Building::Mill(Tile::Farm)));
        builds.push(Secondary::Build(Building::Mill(Tile::Village)));
    }

    builds
}

fn get_deploys(state: &PlayerState) -> Vec<Secondary> {
    let mut deploys = Vec::new();
    if !state.mechs.first_deployed {
        deploys.push(Secondary::Deploy(Mech::First));
    }
    if !state.mechs.second_deployed {
        deploys.push(Secondary::Deploy(Mech::Second));
    }
    if !state.mechs.third_deployed {
        deploys.push(Secondary::Deploy(Mech::Third));
    }
    if !state.mechs.fourth_deployed {
        deploys.push(Secondary::Deploy(Mech::Fourth));
    }
    deploys
}

const PRIMARY_UPGRADES: [PrimaryUpgrade; 6] = [
    PrimaryUpgrade::Move,
    PrimaryUpgrade::Produce,
    PrimaryUpgrade::Bolster,
    PrimaryUpgrade::Enforce,
    PrimaryUpgrade::Promote,
    PrimaryUpgrade::Tax,
];

const SECONDARY_UPGRADES: [SecondaryUpgrade; 4] = [
    SecondaryUpgrade::Build,
    SecondaryUpgrade::Deploy,
    SecondaryUpgrade::Enlist,
    SecondaryUpgrade::Upgrade,
];

fn get_upgrades(state: &PlayerState) -> Vec<Secondary> {
    let primary = PRIMARY_UPGRADES
        .iter()
        .filter(|&upgrade| state.upgrades.can_upgrade_primary(upgrade))
        .collect::<Vec<&PrimaryUpgrade>>();
    let secondary = SECONDARY_UPGRADES
        .iter()
        .filter(|&upgrade| state.upgrades.can_upgrade_secondary(upgrade))
        .collect::<Vec<&SecondaryUpgrade>>();

    let mut upgrades = Vec::new();
    for prim in primary {
        for sec in secondary.clone() {
            upgrades.push(Secondary::Upgrade(*prim, *sec));
        }
    }

    upgrades
}

fn get_moves(state: &PlayerState) -> Vec<Primary> {
    // todo: this is not the full action space
    let mut moves = Vec::new();
    for tile1 in vec![
        Tile::Woods,
        Tile::Mountain,
        Tile::Tundra,
        Tile::Farm,
        Tile::Village,
    ] {
        let prod = state.production.get(&tile1);
        if prod <= 0 {
            continue;
        }
        for tile2 in vec![
            Tile::Woods,
            Tile::Mountain,
            Tile::Tundra,
            Tile::Farm,
            Tile::Village,
        ] {
            if tile1 == tile2 {
                continue;
            }
            moves.push(Primary::Move(Move::Move1((tile1, tile2))));
            if prod >= 2 {
                moves.push(Primary::Move(Move::Move2((tile1, tile2), (tile1, tile2))));
            }
            if state.upgrades.move_evolved && prod >= 3 {
                moves.push(Primary::Move(Move::Move3(
                    (tile1, tile2),
                    (tile1, tile2),
                    (tile1, tile2),
                )));
            }
        }
    }

    moves
}

const TILES: [Tile; 5] = [
    Tile::Woods,
    Tile::Mountain,
    Tile::Tundra,
    Tile::Farm,
    Tile::Village,
];

fn get_produces(state: &PlayerState) -> Vec<Primary> {
    let tiles = TILES
        .iter()
        .filter(|&tile| state.production.get(tile) > 0)
        .collect::<Vec<&Tile>>();

    let mut produces = Vec::new();
    if state.upgrades.produce_evolved {
        for i in 0..tiles.len() - 2 {
            for j in i + 1..tiles.len() - 1 {
                for k in j + 1..tiles.len() {
                    produces.push(Primary::Produce(Produce::Produce3(
                        *tiles[i], *tiles[j], *tiles[k],
                    )));
                }
            }
        }
    }
    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            produces.push(Primary::Produce(Produce::Produce2(*tiles[i], *tiles[j])));
        }
    }

    for tile in tiles {
        produces.push(Primary::Produce(Produce::Produce1(*tile)));
    }

    produces
}

fn get_trades() -> Vec<Primary> {
    let mut trades = Vec::new();
    for res1 in vec![
        Resource::Wood,
        Resource::Metal,
        Resource::Oil,
        Resource::Food,
    ] {
        for res2 in vec![
            Resource::Wood,
            Resource::Metal,
            Resource::Oil,
            Resource::Food,
        ] {
            trades.push(Primary::Trade(res1, res2));
        }
    }
    trades
}
