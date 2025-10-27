use std::{cmp::min, rc::Rc};

use crate::{
    game::{
        board::{Field, ResourceField},
        buildings::BuildingsState,
        character::CharacterEntity,
        mechs::MechsState,
        military::MilitaryState,
        popularity::PopularityState,
        production::ProductionState,
        recruits::RecruitsState,
        upgrades::UpgradesState,
    },
    template::{Faction, Player, PlayerMat, PrimaryAction, SecondaryAction},
};

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub move_secondary: SecondaryAction, // for move and tax primary actions
    pub trade_secondary: SecondaryAction, // for trade and promote primary actions
    pub produce_secondary: SecondaryAction, // for produce primary action
    pub bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    pub upgrades: UpgradesState,
    pub mechs: MechsState,
    pub buildings: BuildingsState,
    pub recruits: RecruitsState,
    pub military: MilitaryState,
    pub popularity: PopularityState,
    pub production: ProductionState,
    pub character: CharacterEntity,

    pub coins: u32,
    pub cards: u8, // TODO: change to actual BattleCards
    pub combat_wins: u8,
}

#[derive(Debug, Clone)]
pub struct PlayerTemplate<'a> {
    pub player: Player<'a>,
    pub faction: Faction<'a>,
    pub player_mat: PlayerMat<'a>,
}

pub type Territory = Vec<Rc<Field>>;

impl PlayerState {
    pub fn new(
        template: &PlayerTemplate,
        home: &Rc<Field>,
        first: &Rc<Field>,
        second: &Rc<Field>,
    ) -> PlayerState {
        let mut production = ProductionState::new();
        production.deploy(first);
        production.deploy(second);

        PlayerState {
            move_secondary: template.player_mat.move_secondary,
            trade_secondary: template.player_mat.trade_secondary,
            produce_secondary: template.player_mat.produce_secondary,
            bolster_secondary: template.player_mat.bolster_secondary,

            upgrades: UpgradesState::new(&template.player_mat),
            mechs: MechsState::new(),
            buildings: BuildingsState::new(),
            recruits: RecruitsState::new(),
            military: MilitaryState::new(
                template.faction.starting_power + template.player.bonus_starting_power,
            ),
            popularity: PopularityState::new(
                template.player_mat.starting_popularity + template.player.bonus_starting_popularity,
            ),
            production: production,
            character: CharacterEntity {
                location: home.clone(),
            },

            coins: template.player_mat.starting_coins + template.player.bonus_starting_coins,
            cards: template.faction.starting_cards,
            combat_wins: 0,
        }
    }

    pub fn stars(&self) -> u8 {
        let mut stars = 0;
        if self.upgrades.star { stars += 1 }
        if self.mechs.star { stars += 1 }
        if self.buildings.star { stars += 1 }
        if self.recruits.star { stars += 1 }
        if self.production.star { stars += 1 }
        // TODO Objectives
        stars += min(self.combat_wins, 2);
        if self.popularity.star { stars += 1 }
        if self.military.star { stars += 1 }
        stars
    }

    pub fn has_won(&self) -> bool {
        self.stars() >= 6
    }

    pub fn controlled_territory(&self) -> Territory {
        let mut fields = [const { None }; 17];
        fields[0] = Some(self.character.location.clone());
        fields[1..9].clone_from_slice(self.production.workers.as_slice());
        fields[9..13].clone_from_slice(self.mechs.mechs.as_slice());
        fields[13].clone_from(&self.buildings.armory);
        fields[14].clone_from(&self.buildings.mill);
        fields[15].clone_from(&self.buildings.mine);
        fields[16].clone_from(&self.buildings.monument);

        // TODO buildings only count towards controlled territory, if no enemy unit is currently on that fields

        let mut territory = Vec::new();
        for ele in fields {
            if let Some(tile) = ele {
                if !territory.contains(&tile) {
                    territory.push(tile);
                }
            }
        }

        territory
    }

    pub fn territory_size(&self, territory: Option<&Territory>) -> u32 {
        let terr = match territory {
            Some(t) => t,
            None => &self.controlled_territory(),
        };
        terr.len() as u32
    }

    pub fn recources(&self, territory: Option<&Territory>) -> ResourceField {
        let terr = match territory {
            Some(t) => t,
            None => &self.controlled_territory(),
        };
        terr.iter().fold(
            ResourceField {
                wood: 0,
                metal: 0,
                oil: 0,
                food: 0,
            },
            |a, b| a + b.resources,
        )
    }

    pub fn total_coins(&self) -> u32 {
        let territory = self.controlled_territory();
        let fields_amount = self.territory_size(Some(&territory));
        let resources = self.recources(Some(&territory));

        self.coins
            + u32::from(self.stars()).saturating_mul(self.popularity.star_multiplier().into())
            + fields_amount.saturating_mul(self.popularity.fields_multiplier().into())
            + (resources.total() / 2).saturating_mul(self.popularity.resources_multiplier().into())
    }

    pub fn get_primary(&self, secondary: SecondaryAction) -> PrimaryAction {
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

    pub fn get_secondary(&self, primary: PrimaryAction) -> SecondaryAction {
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

    pub fn can_produce(&self) -> bool {
        let total = self.production.deployed_workers;
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
}
