use std::slice::Iter;

use buildings::BuildingsState;
use mechs::MechsState;
use military::MilitaryState;
use popularity::PopularityState;
use production::ProductionState;
use recruits::RecruitsState;
use resources::ResourcesState;
use upgrades::UpgradesState;

use crate::{
    campaign::{Faction, Player, PlayerMat, PrimaryAction, SecondaryAction},
    game::turnmask::TurnMask,
};

pub mod buildings;
pub mod mechs;
pub mod military;
pub mod popularity;
pub mod production;
pub mod recruits;
pub mod resources;
pub mod upgrades;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PlayerState {
    pub(crate) move_secondary: SecondaryAction, // for move and tax primary actions
    pub(crate) trade_secondary: SecondaryAction, // for trade and promote primary actions
    pub(crate) produce_secondary: SecondaryAction, // for produce primary action
    pub(crate) bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    pub(crate) upgrades: UpgradesState,
    pub(crate) mechs: MechsState,
    pub(crate) buildings: BuildingsState,
    pub(crate) recruits: RecruitsState,
    pub(crate) military: MilitaryState,
    pub(crate) popularity: PopularityState,
    pub(crate) production: ProductionState,

    pub(crate) resources: ResourcesState,
    pub(crate) coins: i32,
    pub(crate) fields: i32,
    pub(crate) cards: i32, // TODO: change to actual BattleCards
    pub(crate) turns: i32,
}

impl PlayerState {
    pub(crate) fn new<'a>(
        player: &'a Player<'a>,
        faction: &'a Faction<'a>,
        player_mat: &'a PlayerMat<'a>,
    ) -> PlayerState {
        PlayerState {
            move_secondary: player_mat.move_secondary,
            trade_secondary: player_mat.trade_secondary,
            produce_secondary: player_mat.produce_secondary,
            bolster_secondary: player_mat.bolster_secondary,

            upgrades: UpgradesState::new(&player_mat),
            mechs: MechsState::new(),
            buildings: BuildingsState::new(),
            recruits: RecruitsState::new(),
            military: MilitaryState::new(faction.starting_power + player.bonus_starting_power),
            popularity: PopularityState::new(
                player_mat.starting_popularity + player.bonus_starting_popularity,
            ),
            production: ProductionState::new(vec![
                faction.first_starting_field,
                faction.second_starting_field,
            ]),

            resources: ResourcesState::new(),
            coins: player_mat.starting_coins + player.bonus_starting_coins,
            fields: 2,
            cards: faction.starting_cards,
            turns: 0,
        }
    }

    pub(crate) fn stars(&self) -> i32 {
        [
            self.upgrades.star,
            self.mechs.star,
            self.buildings.star,
            self.recruits.star,
            self.military.star,
            self.popularity.star,
        ]
        .iter()
        .fold(0, |acc, &star| acc + if star { 1 } else { 0 })
    }

    pub(crate) fn has_won(&self) -> bool {
        self.stars() >= 6
    }

    pub(crate) fn total_coins(&self) -> i32 {
        self.coins
            + self.stars() * self.popularity.star_multiplier()
            + self.fields * self.popularity.fields_multiplier()
            + self.resources.total() / 2 * self.popularity.resources_multiplier()
    }

    pub(crate) fn get_primary(&self, secondary: SecondaryAction) -> PrimaryAction {
        if secondary == self.move_secondary {
            return PrimaryAction::Tax;
        } else if secondary == self.trade_secondary {
            return PrimaryAction::Promote;
        } else if secondary == self.produce_secondary {
            return PrimaryAction::Produce;
        } else if secondary == self.bolster_secondary {
            return PrimaryAction::Bolster;
        } else {
            panic!("Invalid secondary action: {:?} is not linked", secondary);
        }
    }

    pub(crate) fn get_secondary(&self, primary: PrimaryAction) -> SecondaryAction {
        match primary {
            PrimaryAction::Move => self.move_secondary,
            PrimaryAction::Tax => self.move_secondary,
            PrimaryAction::Trade => self.trade_secondary,
            PrimaryAction::Promote => self.trade_secondary,
            PrimaryAction::Bolster => self.bolster_secondary,
            PrimaryAction::Enforce => self.bolster_secondary,
            PrimaryAction::Produce => self.produce_secondary,
        }
    }

    pub(crate) fn can_produce(&self) -> bool {
        let total = self.production.total();
        if total >= 8 && self.coins <= 0 {
            return false;
        }
        if total >= 6 && self.popularity.popularity <= 0 {
            return false;
        }
        if total >= 4 && self.military.power <= 0 {
            return false;
        }
        true
    }

    pub(crate) fn get_actions(&self) -> Iter<'_, TurnMask> {
        todo!()
    }
}
