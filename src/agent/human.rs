use crate::{
    campaign::{PrimaryAction, SecondaryAction},
    game_state::{
        buildings::Building, mechs::Mech, recruits::Recruit, resources::Resource,
        upgrades::Upgrade, PlayerState,
    },
};

use super::Agent;

pub(crate) struct FullAgentIndustrialRusviet {
    wanted: Option<Resource>,
}

impl FullAgentIndustrialRusviet {
    pub(crate) fn new() -> Self {
        Self {
            wanted: Some(Resource::People),
        }
    }
}

impl Agent for FullAgentIndustrialRusviet {
    fn prepare(&mut self, state: &PlayerState) {
        if state.production.total() < 5 {
            self.wanted = Some(Resource::People);
        } else if !state.upgrades.star {
            self.wanted = Some(Resource::Oil);
        } else if !state.buildings.star {
            self.wanted = Some(Resource::Wood);
        } else if !state.recruits.star {
            self.wanted = Some(Resource::Food);
        } else if !state.mechs.star {
            self.wanted = Some(Resource::Metal);
        } else {
            self.wanted = None;
        }
    }

    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction {
        // bolster_secondary: SecondaryAction::Upgrade,
        // move_secondary: SecondaryAction::Build,
        // produce_secondary: SecondaryAction::Deploy,
        // trade_secondary: SecondaryAction::Enlist,

        // Make sure we have enough to do the primary action
        if state.coins <= 0 {
            return PrimaryAction::Tax;
        }

        if state.military.power <= 1 {
            return PrimaryAction::Bolster;
        }

        if state.popularity.popularity <= 1 {
            return PrimaryAction::Promote;
        }

        match self.wanted {
            Some(Resource::People) => PrimaryAction::Produce,
            Some(Resource::Oil) => {
                if state.resources.oil < state.upgrades.get_upgrade_cost(SecondaryAction::Upgrade) {
                    if state.production.oil < state.production.total() {
                        PrimaryAction::Move
                    } else {
                        PrimaryAction::Produce
                    }
                } else {
                    PrimaryAction::Bolster
                }
            }
            Some(Resource::Wood) => {
                if state.resources.wood < state.upgrades.get_upgrade_cost(SecondaryAction::Build) {
                    if state.production.wood < state.production.total() {
                        PrimaryAction::Move
                    } else {
                        PrimaryAction::Produce
                    }
                } else {
                    PrimaryAction::Tax
                }
            }
            Some(Resource::Metal) => {
                if state.resources.metal < state.upgrades.get_upgrade_cost(SecondaryAction::Deploy)
                {
                    if state.production.metal < state.production.total() {
                        PrimaryAction::Move
                    } else {
                        PrimaryAction::Produce
                    }
                } else {
                    PrimaryAction::Produce
                }
            }
            Some(Resource::Food) => {
                if state.resources.food < state.upgrades.get_upgrade_cost(SecondaryAction::Enlist) {
                    if state.production.food < state.production.total() {
                        PrimaryAction::Move
                    } else {
                        PrimaryAction::Produce
                    }
                } else {
                    PrimaryAction::Promote
                }
            }
            _ if !state.military.star => PrimaryAction::Bolster,
            _ if !state.popularity.star => PrimaryAction::Promote,
            _ => PrimaryAction::Tax,
        }
    }

    fn choose_trade(&self, _state: &PlayerState) -> Resource {
        match self.wanted {
            Some(resource) => resource,
            _ => Resource::Metal,
        }
    }

    fn choose_produce(&self, _state: &PlayerState) -> Resource {
        match self.wanted {
            Some(resource) => resource,
            _ => Resource::Metal,
        }
    }

    // Move the first available resource to the wanted resource
    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)> {
        match self.wanted {
            Some(wanted) => {
                let from = vec![
                    Resource::Wood,
                    Resource::Metal,
                    Resource::Oil,
                    Resource::Food,
                    Resource::People,
                ]
                .into_iter()
                .filter(|resource| state.production.get(*resource) > 0)
                .filter(|resource| resource != &wanted)
                .next();

                match from {
                    Some(from) => Some((from, wanted)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // Upgrade the first available primary and secondary; prioritize production and upgrades
    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)> {
        let primary_list = vec![
            Upgrade::Produce,
            Upgrade::Move,
            Upgrade::Popularity,
            Upgrade::Power,
            Upgrade::Tax,
            Upgrade::Card,
        ]
        .into_iter()
        .filter(|upgrade| state.upgrades.can_upgrade_primary(*upgrade))
        .next();

        let secondary_list = vec![
            SecondaryAction::Upgrade,
            SecondaryAction::Build,
            SecondaryAction::Enlist,
            SecondaryAction::Deploy,
        ]
        .into_iter()
        .filter(|secondary| state.upgrades.can_upgrade_secondary(*secondary))
        .next();

        match (primary_list, secondary_list) {
            (Some(primary), Some(secondary)) => Some((primary, secondary)),
            _ => None,
        }
    }

    // Deploy the first available mech
    fn deploy(&self, state: &PlayerState) -> Option<Mech> {
        let mech_list = vec![Mech::First, Mech::Second, Mech::Third, Mech::Fourth]
            .into_iter()
            .filter(|mech| state.mechs.can_deploy(*mech))
            .next();

        match mech_list {
            Some(mech) => Some(mech),
            _ => None,
        }
    }

    // Build the next building, first the mill
    fn build(&self, state: &PlayerState) -> Option<Building> {
        let building_list = vec![
            Building::Mill,
            Building::Mine,
            Building::Armory,
            Building::Monument,
        ]
        .into_iter()
        .filter(|building| state.buildings.can_build(*building))
        .next();

        match building_list {
            Some(building) => Some(building),
            _ => None,
        }
    }

    // Build on metal to try and get it passively
    fn choose_mill_location(&self, _state: &PlayerState) -> Resource {
        Resource::Metal
    }

    // Enlist the first available recruits
    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)> {
        let secondary_list = vec![
            Recruit::Coin,
            Recruit::Popularity,
            Recruit::Military,
            Recruit::Card,
        ]
        .into_iter()
        .filter(|recruit| !state.recruits.is_secondary_recruited(*recruit))
        .next();

        let onetime_list = vec![
            Recruit::Popularity,
            Recruit::Coin,
            Recruit::Military,
            Recruit::Card,
        ]
        .into_iter()
        .filter(|recruit| !state.recruits.is_onetime_recruited(*recruit))
        .next();

        match (secondary_list, onetime_list) {
            (Some(secondary), Some(onetime)) => Some((secondary, onetime)),
            _ => None,
        }
    }
}
