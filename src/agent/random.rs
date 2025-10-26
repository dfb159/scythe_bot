use rand::seq::SliceRandom;
use rand::{prelude::ThreadRng, seq::IteratorRandom};

use crate::{
    template::{PrimaryAction, SecondaryAction},
    game::{
        turnhelper::{check_secondary_cost, map_primary},
        turnmask::{
            Building, Mech,
            Move::{Move1, Move2, Move3},
            Primary, PrimaryUpgrade, Produce, Recruit, Resource, Secondary, SecondaryUpgrade, Tile,
            TurnMask,
        },
    },
    game_state::PlayerState,
};

use super::Agent;

pub(crate) struct RandomAgent {
    rng: ThreadRng,
}
impl RandomAgent {
    pub(crate) fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    fn choose_primary(&mut self, state: &PlayerState) -> Primary {
        let mut actions = vec![];
        actions.push(PrimaryAction::Tax);
        if state.production.total() > 0 {
            actions.push(PrimaryAction::Move);
        }
        if state.coins > 0 {
            actions.push(PrimaryAction::Trade);
            actions.push(PrimaryAction::Promote);
            actions.push(PrimaryAction::Bolster);
            actions.push(PrimaryAction::Enforce);
        }
        if state.can_produce() {
            actions.push(PrimaryAction::Produce);
        }

        let action = actions.iter().choose(&mut self.rng).unwrap();

        match action {
            PrimaryAction::Tax => Primary::Tax,
            PrimaryAction::Move => {
                let mut tiles = Vec::with_capacity(state.production.total() as usize);
                tiles.extend(vec![
                    Tile::Woods;
                    state.production.get(&Tile::Woods) as usize
                ]);
                tiles.extend(vec![Tile::Farm; state.production.get(&Tile::Farm) as usize]);
                tiles.extend(vec![
                    Tile::Mountain;
                    state.production.get(&Tile::Mountain) as usize
                ]);
                tiles.extend(vec![
                    Tile::Tundra;
                    state.production.get(&Tile::Tundra) as usize
                ]);
                tiles.extend(vec![
                    Tile::Village;
                    state.production.get(&Tile::Village) as usize
                ]);

                tiles.shuffle(&mut self.rng);
                let mode = tiles
                    .len()
                    .min(if state.upgrades.move_evolved { 3 } else { 2 });
                match (0..mode).choose(&mut self.rng).unwrap() {
                    1 => Primary::Move(Move2(self.move_tile(tiles[0]), self.move_tile(tiles[1]))),
                    2 => Primary::Move(Move3(
                        self.move_tile(tiles[0]),
                        self.move_tile(tiles[1]),
                        self.move_tile(tiles[2]),
                    )),
                    _ => Primary::Move(Move1(self.move_tile(tiles[0]))),
                }
            }
            PrimaryAction::Trade => Primary::Trade(
                *vec![
                    Resource::Wood,
                    Resource::Oil,
                    Resource::Metal,
                    Resource::Food,
                ]
                .iter()
                .choose(&mut self.rng)
                .unwrap(),
                *vec![
                    Resource::Wood,
                    Resource::Oil,
                    Resource::Metal,
                    Resource::Food,
                ]
                .iter()
                .choose(&mut self.rng)
                .unwrap(),
            ),
            PrimaryAction::Produce => {
                let mut tiles = Vec::new();
                if state.production.get(&Tile::Woods) > 0 {
                    tiles.push(Tile::Woods);
                }
                if state.production.get(&Tile::Farm) > 0 {
                    tiles.push(Tile::Farm);
                }
                if state.production.get(&Tile::Mountain) > 0 {
                    tiles.push(Tile::Mountain);
                }
                if state.production.get(&Tile::Tundra) > 0 {
                    tiles.push(Tile::Tundra);
                }
                if state.production.get(&Tile::Village) > 0 {
                    tiles.push(Tile::Village);
                }

                let mode = tiles
                    .len()
                    .min(if state.upgrades.produce_evolved { 3 } else { 2 });
                match (0..mode).choose(&mut self.rng).unwrap() {
                    1 => Primary::Produce(Produce::Produce2(tiles[0], tiles[1])),
                    2 => Primary::Produce(Produce::Produce3(tiles[0], tiles[1], tiles[2])),
                    _ => Primary::Produce(Produce::Produce1(tiles[0])),
                }
            }
            PrimaryAction::Bolster => Primary::Bolster,
            PrimaryAction::Enforce => Primary::Enforce,
            PrimaryAction::Promote => Primary::Promote,
        }
    }

    fn choose_secondary(
        &mut self,
        state: &PlayerState,
        primary: PrimaryAction,
    ) -> Option<Secondary> {
        let action = state.get_secondary(primary);
        if check_secondary_cost(state, &action) {
            match action {
                SecondaryAction::Build => {
                    let mut possible = vec![];
                    if !state.buildings.monument_built {
                        possible.push(Secondary::Build(Building::Monument));
                    }
                    if !state.buildings.armory_built {
                        possible.push(Secondary::Build(Building::Armory));
                    }
                    if !state.buildings.mine_built {
                        possible.push(Secondary::Build(Building::Tunnel));
                    }
                    if !state.buildings.mill_built {
                        let location = LOCATIONS.iter().choose(&mut self.rng).unwrap();
                        possible.push(Secondary::Build(Building::Mill(*location)));
                    }

                    if possible.is_empty() {
                        None
                    } else {
                        Some(possible.iter().choose(&mut self.rng).unwrap().clone())
                    }
                }
                SecondaryAction::Deploy => {
                    let mut possible = vec![];
                    if !state.mechs.first_deployed {
                        possible.push(Secondary::Deploy(Mech::First));
                    }
                    if !state.mechs.second_deployed {
                        possible.push(Secondary::Deploy(Mech::Second));
                    }
                    if !state.mechs.third_deployed {
                        possible.push(Secondary::Deploy(Mech::Third));
                    }
                    if !state.mechs.fourth_deployed {
                        possible.push(Secondary::Deploy(Mech::Fourth));
                    }

                    if possible.is_empty() {
                        None
                    } else {
                        Some(possible.iter().choose(&mut self.rng).unwrap().clone())
                    }
                }
                SecondaryAction::Enlist => {
                    let mut possible = vec![];
                    if !state.recruits.secondary_card_recruited {
                        possible.push(Recruit::Card);
                    }
                    if !state.recruits.secondary_coin_recruited {
                        possible.push(Recruit::Coin);
                    }
                    if !state.recruits.secondary_military_recruited {
                        possible.push(Recruit::Power);
                    }
                    if !state.recruits.secondary_popularity_recruited {
                        possible.push(Recruit::Popularity);
                    }
                    let secondary = possible.iter().choose(&mut self.rng);

                    let mut possible = vec![];
                    if !state.recruits.onetime_card_recruited {
                        possible.push(Recruit::Card);
                    }
                    if !state.recruits.onetime_coin_recruited {
                        possible.push(Recruit::Coin);
                    }
                    if !state.recruits.onetime_military_recruited {
                        possible.push(Recruit::Power);
                    }
                    if !state.recruits.onetime_popularity_recruited {
                        possible.push(Recruit::Popularity);
                    }
                    let onetime = possible.iter().choose(&mut self.rng);

                    match (secondary, onetime) {
                        (Some(secondary), Some(onetime)) => {
                            Some(Secondary::Enlist(*secondary, *onetime))
                        }
                        _ => None,
                    }
                }
                SecondaryAction::Upgrade => {
                    let mut possible = vec![];
                    if !state.upgrades.move_evolved {
                        possible.push(PrimaryUpgrade::Move);
                    }
                    if !state.upgrades.produce_evolved {
                        possible.push(PrimaryUpgrade::Produce);
                    }
                    if !state.upgrades.tax_evolved {
                        possible.push(PrimaryUpgrade::Tax);
                    }
                    if !state.upgrades.power_evolved {
                        possible.push(PrimaryUpgrade::Bolster);
                    }
                    if !state.upgrades.card_evolved {
                        possible.push(PrimaryUpgrade::Enforce);
                    }
                    if !state.upgrades.popularity_evolved {
                        possible.push(PrimaryUpgrade::Promote);
                    }

                    let primary = possible.iter().choose(&mut self.rng);

                    let mut possible = vec![];
                    if !state.upgrades.upgrade_evolution_cost > 0 {
                        possible.push(SecondaryUpgrade::Upgrade);
                    }
                    if !state.upgrades.deploy_evolution_cost > 0 {
                        possible.push(SecondaryUpgrade::Deploy);
                    }
                    if !state.upgrades.build_evolution_cost > 0 {
                        possible.push(SecondaryUpgrade::Build);
                    }
                    if !state.upgrades.enlist_evolution_cost > 0 {
                        possible.push(SecondaryUpgrade::Enlist);
                    }
                    let secondary = possible.iter().choose(&mut self.rng);

                    match (primary, secondary) {
                        (Some(primary), Some(secondary)) => {
                            Some(Secondary::Upgrade(*primary, *secondary))
                        }
                        _ => None,
                    }
                }
            }
        } else {
            None
        }
    }

    fn move_tile(&self, tile: Tile) -> (Tile, Tile) {
        match tile {
            Tile::Woods => (Tile::Woods, Tile::Woods),
            Tile::Tundra => (Tile::Tundra, Tile::Tundra),
            Tile::Mountain => (Tile::Mountain, Tile::Mountain),
            Tile::Farm => (Tile::Farm, Tile::Farm),
            Tile::Village => (Tile::Village, Tile::Village),
        }
    }
}

impl Agent for RandomAgent {
    fn get_action(&mut self, state: &PlayerState) -> TurnMask {
        let primary = self.choose_primary(state);
        let secondary = self.choose_secondary(state, map_primary(&primary));
        match secondary {
            None => TurnMask::PrimaryOnly(primary),
            Some(sec) => TurnMask::PrimaryAndSecondary(primary, sec),
        }
    }
}

const LOCATIONS: [Tile; 5] = [
    Tile::Woods,
    Tile::Mountain,
    Tile::Tundra,
    Tile::Farm,
    Tile::Village,
];
