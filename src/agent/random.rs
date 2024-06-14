use rand::{seq::IteratorRandom, Rng};

use crate::{
    campaign::{PrimaryAction, SecondaryAction},
    game_state::{
        buildings::Building, mechs::Mech, recruits::Recruit, resources::Resource,
        upgrades::Upgrade, PlayerState,
    },
};

use super::Agent;

pub(crate) struct RandomAgent {}

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
