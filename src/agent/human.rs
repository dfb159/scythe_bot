use crate::{
    template::{PrimaryAction, SecondaryAction},
    game::{
        turnhelper::{check_secondary_cost, map_primary, map_tile_resource},
        turnmask::{
            Building, Mech, Move, Primary, PrimaryUpgrade, Produce, Recruit, Resource, Secondary,
            SecondaryUpgrade, Tile, TurnMask,
        },
    },
    game_state::PlayerState,
};

use super::Agent;

#[derive(Debug)]
pub enum Step {
    Population,
    Power,
    Popularity,

    Upgrade,
    Deploy,
    Build,
    Recruit,
}

pub struct PriorityAgent {
    pub priority: Vec<Step>,
    pub final_step: PrimaryAction,
    pub mill_tile: Tile,
}
impl PriorityAgent {
    fn choose_primary(&self, state: &PlayerState) -> Primary {
        let step = self.priority.iter().find(|step| match step {
            Step::Population => !state.production.star,
            Step::Power => !state.military.star,
            Step::Popularity => !state.popularity.star,
            Step::Upgrade => !state.upgrades.star,
            Step::Deploy => !state.mechs.star,
            Step::Build => !state.buildings.star,
            Step::Recruit => !state.recruits.star,
        });

        match step {
            Some(Step::Population) => move_produce(state, &Tile::Village, 8),
            Some(Step::Power) => create_boster(state),
            Some(Step::Popularity) => Primary::Promote,
            Some(Step::Upgrade) => {
                let cost = state.upgrades.get_upgrade_cost(&SecondaryAction::Upgrade);
                if cost <= state.resources.get(Resource::Oil) {
                    // choose the primary action that enables upgrading as the secondary action
                    create_primary_linked(state, SecondaryAction::Upgrade, &Tile::Tundra)
                } else {
                    // try to produce the wanted amount
                    move_produce(state, &Tile::Tundra, cost)
                }
            }
            Some(Step::Deploy) => {
                let cost = state.upgrades.get_upgrade_cost(&SecondaryAction::Deploy);
                if cost <= state.resources.get(Resource::Metal) {
                    // choose the primary action that enables deploying as the secondary action
                    create_primary_linked(state, SecondaryAction::Deploy, &Tile::Mountain)
                } else {
                    // try to produce the wanted amount
                    move_produce(state, &Tile::Mountain, cost)
                }
            }
            Some(Step::Build) => {
                let cost = state.upgrades.get_upgrade_cost(&SecondaryAction::Build);
                if cost <= state.resources.get(Resource::Wood) {
                    // choose the primary action that enables building as the secondary action
                    create_primary_linked(state, SecondaryAction::Build, &Tile::Woods)
                } else {
                    // try to produce the wanted amount
                    move_produce(state, &Tile::Woods, cost)
                }
            }
            Some(Step::Recruit) => {
                let cost = state.upgrades.get_upgrade_cost(&SecondaryAction::Enlist);
                if cost <= state.resources.get(Resource::Food) {
                    // choose the primary action that enables enlisting as the secondary action
                    create_primary_linked(state, SecondaryAction::Enlist, &Tile::Farm)
                } else {
                    // try to produce the wanted amount
                    move_produce(state, &Tile::Farm, cost)
                }
            }
            _ => match self.final_step {
                PrimaryAction::Move => match create_move(state, &Tile::Woods) {
                    Some(m) => Primary::Move(m),
                    None => Primary::Move(Move::Move1((Tile::Woods, Tile::Mountain))),
                },
                PrimaryAction::Tax => Primary::Tax,
                PrimaryAction::Trade => create_trade(state, Resource::Wood),
                PrimaryAction::Promote => create_promote(state),
                PrimaryAction::Bolster => create_bolster(state),
                PrimaryAction::Enforce => create_enforce(state),
                PrimaryAction::Produce => create_produce(state, &Tile::Woods),
            },
        }
    }

    fn choose_secondary(
        &self,
        state: &PlayerState,
        secondary: SecondaryAction,
    ) -> Option<Secondary> {
        match secondary {
            SecondaryAction::Upgrade => {
                let primary_upgrade = if !state.upgrades.move_evolved {
                    Some(PrimaryUpgrade::Move)
                } else if !state.upgrades.tax_evolved {
                    Some(PrimaryUpgrade::Tax)
                } else if !state.upgrades.popularity_evolved {
                    Some(PrimaryUpgrade::Promote)
                } else if !state.upgrades.produce_evolved {
                    Some(PrimaryUpgrade::Produce)
                } else if !state.upgrades.power_evolved {
                    Some(PrimaryUpgrade::Bolster)
                } else if !state.upgrades.card_evolved {
                    Some(PrimaryUpgrade::Enforce)
                } else {
                    return None;
                };

                let secondary_upgrade = if state.upgrades.upgrade_evolution_cost > 0 {
                    Some(SecondaryUpgrade::Upgrade)
                } else if state.upgrades.deploy_evolution_cost > 0 {
                    Some(SecondaryUpgrade::Deploy)
                } else if state.upgrades.build_evolution_cost > 0 {
                    Some(SecondaryUpgrade::Build)
                } else if state.upgrades.enlist_evolution_cost > 0 {
                    Some(SecondaryUpgrade::Enlist)
                } else {
                    return None;
                };

                match (primary_upgrade, secondary_upgrade) {
                    (Some(primary), Some(secondary)) => {
                        Some(Secondary::Upgrade(primary, secondary))
                    }
                    _ => None,
                }
            }
            SecondaryAction::Deploy => {
                let mech = if !state.mechs.first_deployed {
                    Some(Mech::First)
                } else if !state.mechs.second_deployed {
                    Some(Mech::Second)
                } else if !state.mechs.third_deployed {
                    Some(Mech::Third)
                } else if !state.mechs.fourth_deployed {
                    Some(Mech::Fourth)
                } else {
                    return None;
                };

                match mech {
                    Some(mech) => Some(Secondary::Deploy(mech)),
                    _ => None,
                }
            }
            SecondaryAction::Build => {
                let building = if !state.buildings.mill_built {
                    Some(Building::Mill(self.mill_tile))
                } else if !state.buildings.armory_built {
                    Some(Building::Armory)
                } else if !state.buildings.monument_built {
                    Some(Building::Monument)
                } else if !state.buildings.mine_built {
                    Some(Building::Tunnel)
                } else {
                    return None;
                };

                match building {
                    Some(building) => Some(Secondary::Build(building)),
                    _ => None,
                }
            }
            SecondaryAction::Enlist => {
                let secondary = if !state.recruits.secondary_coin_recruited {
                    Some(Recruit::Coin)
                } else if !state.recruits.secondary_military_recruited {
                    Some(Recruit::Power)
                } else if !state.recruits.secondary_popularity_recruited {
                    Some(Recruit::Popularity)
                } else if !state.recruits.secondary_card_recruited {
                    Some(Recruit::Card)
                } else {
                    return None;
                };

                let onetime = if !state.recruits.onetime_coin_recruited {
                    Some(Recruit::Coin)
                } else if !state.recruits.onetime_military_recruited {
                    Some(Recruit::Power)
                } else if !state.recruits.onetime_popularity_recruited {
                    Some(Recruit::Popularity)
                } else if !state.recruits.onetime_card_recruited {
                    Some(Recruit::Card)
                } else {
                    return None;
                };

                match (secondary, onetime) {
                    (Some(secondary), Some(onetime)) => Some(Secondary::Enlist(secondary, onetime)),
                    _ => None,
                }
            }
        }
    }
}

fn create_enforce(state: &PlayerState) -> Primary {
    if state.coins <= 0 {
        Primary::Tax
    } else {
        Primary::Enforce
    }
}

fn create_trade(state: &PlayerState, resource: Resource) -> Primary {
    if state.coins <= 0 {
        Primary::Tax
    } else {
        Primary::Trade(resource, resource)
    }
}

fn create_boster(state: &PlayerState) -> Primary {
    if state.coins <= 0 {
        Primary::Tax
    } else {
        Primary::Bolster
    }
}

fn create_primary_linked(state: &PlayerState, action: SecondaryAction, tile: &Tile) -> Primary {
    match state.get_primary(action) {
        PrimaryAction::Move => Primary::Tax,
        PrimaryAction::Tax => Primary::Tax,
        PrimaryAction::Trade => create_promote(state),
        PrimaryAction::Promote => create_promote(state),
        PrimaryAction::Bolster => create_bolster(state),
        PrimaryAction::Enforce => create_bolster(state),
        PrimaryAction::Produce => create_produce(state, tile),
    }
}

fn create_bolster(state: &PlayerState) -> Primary {
    if state.coins <= 0 {
        Primary::Tax
    } else {
        Primary::Bolster
    }
}

fn create_promote(state: &PlayerState) -> Primary {
    if state.coins <= 0 {
        Primary::Tax
    } else {
        Primary::Promote
    }
}

fn move_produce(state: &PlayerState, target: &Tile, wanted: i32) -> Primary {
    let current = match map_tile_resource(target) {
        Some(resource) => state.resources.get(resource),
        None => state.production.total(),
    };
    if wanted - current <= state.production.get(target) {
        return create_produce(state, target);
    }
    match create_move(state, target) {
        Some(m) => return Primary::Move(m),
        None => create_produce(state, target),
    }
}

fn create_move(state: &PlayerState, target: &Tile) -> Option<Move> {
    let mut source = Vec::new();
    for tile in vec![
        &Tile::Woods,
        &Tile::Mountain,
        &Tile::Tundra,
        &Tile::Farm,
        &Tile::Village,
    ] {
        if tile == target {
            continue;
        }

        let count = state.production.get(tile) as usize;
        source.extend(vec![tile; count]);
    }
    if source.len() >= 3 && state.upgrades.move_evolved {
        Some(Move::Move3(
            (source[0], *target),
            (source[1], *target),
            (source[2], *target),
        ))
    } else if source.len() >= 2 {
        Some(Move::Move2((source[0], *target), (source[1], *target)))
    } else if source.len() >= 1 {
        Some(Move::Move1((source[0], *target)))
    } else {
        None
    }
}

fn create_produce(state: &PlayerState, target: &Tile) -> Primary {
    let total = state.production.total();
    if total >= 8 && state.coins <= 0 {
        Primary::Tax
    } else if total >= 6 && state.popularity.popularity <= 0 {
        if state.coins <= 0 {
            Primary::Tax
        } else {
            Primary::Promote
        }
    } else if total >= 4 && state.military.power <= 0 {
        if state.coins <= 0 {
            Primary::Tax
        } else {
            Primary::Bolster
        }
    } else {
        Primary::Produce(Produce::Produce1(*target))
    }
}

impl Agent for PriorityAgent {
    fn get_action(&mut self, state: &PlayerState) -> TurnMask {
        let primary = self.choose_primary(state);
        let secondary = state.get_secondary(map_primary(&primary));
        if check_secondary_cost(state, &secondary) {
            if let Some(secondary) = self.choose_secondary(state, secondary) {
                return TurnMask::PrimaryAndSecondary(primary, secondary);
            }
        }

        TurnMask::PrimaryOnly(primary)
    }
}
