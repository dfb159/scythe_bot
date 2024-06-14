use crate::{campaign::PlayerMat, SecondaryAction};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Upgrade {
    Popularity,
    Power,
    Card,
    Move,
    Tax,
    Produce,
}

#[derive(Debug)]
pub(crate) struct UpgradesState {
    pub(crate) popularity_evolved: bool,
    pub(crate) power_evolved: bool,
    pub(crate) card_evolved: bool,
    pub(crate) move_evolved: bool,
    pub(crate) tax_evolved: bool,
    pub(crate) produce_evolved: bool,

    pub(crate) upgrade_base_cost: i32,
    pub(crate) upgrade_evolution_cost: i32,
    pub(crate) upgrade_coins: i32,
    pub(crate) deploy_base_cost: i32,
    pub(crate) deploy_evolution_cost: i32,
    pub(crate) deploy_coins: i32,
    pub(crate) build_base_cost: i32,
    pub(crate) build_evolution_cost: i32,
    pub(crate) build_coins: i32,
    pub(crate) enlist_base_cost: i32,
    pub(crate) enlist_evolution_cost: i32,
    pub(crate) enlist_coins: i32,

    pub(crate) star: bool,
}

impl UpgradesState {
    pub(crate) fn new(mat: &PlayerMat) -> UpgradesState {
        UpgradesState {
            popularity_evolved: false,
            power_evolved: false,
            card_evolved: false,
            move_evolved: false,
            tax_evolved: false,
            produce_evolved: false,

            upgrade_base_cost: mat.upgrade_cost,
            upgrade_evolution_cost: mat.upgrade_evolutions,
            upgrade_coins: mat.upgrade_coins,
            deploy_base_cost: mat.deploy_cost,
            deploy_evolution_cost: mat.deploy_evolutions,
            deploy_coins: mat.deploy_coins,
            build_base_cost: mat.build_cost,
            build_evolution_cost: mat.build_evolutions,
            build_coins: mat.build_coins,
            enlist_base_cost: mat.enlist_cost,
            enlist_evolution_cost: mat.enlist_evolutions,
            enlist_coins: mat.enlist_coins,

            star: false,
        }
    }

    pub(crate) fn upgrade(&mut self, primary: Upgrade, secondary: SecondaryAction) {
        match primary {
            Upgrade::Popularity => {
                self.popularity_evolved = true;
            }
            Upgrade::Power => {
                self.power_evolved = true;
            }
            Upgrade::Card => {
                self.card_evolved = true;
            }
            Upgrade::Move => {
                self.move_evolved = true;
            }
            Upgrade::Tax => {
                self.tax_evolved = true;
            }
            Upgrade::Produce => {
                self.produce_evolved = true;
            }
        }
        match secondary {
            SecondaryAction::Upgrade => {
                if self.upgrade_evolution_cost > 0 {
                    self.upgrade_evolution_cost -= 1;
                }
            }
            SecondaryAction::Deploy => {
                if self.deploy_evolution_cost > 0 {
                    self.deploy_evolution_cost -= 1;
                }
            }
            SecondaryAction::Build => {
                if self.build_evolution_cost > 0 {
                    self.build_evolution_cost -= 1;
                }
            }
            SecondaryAction::Enlist => {
                if self.enlist_evolution_cost > 0 {
                    self.enlist_evolution_cost -= 1;
                }
            }
        }
        if self.popularity_evolved
            && self.power_evolved
            && self.card_evolved
            && self.move_evolved
            && self.tax_evolved
            && self.produce_evolved
        {
            self.star = true;
        }
    }

    pub(crate) fn can_upgrade(&self, primary: Upgrade, secondary: SecondaryAction) -> bool {
        self.can_upgrade_primary(primary) && self.can_upgrade_secondary(secondary)
    }

    pub(crate) fn can_upgrade_primary(&self, primary: Upgrade) -> bool {
        match primary {
            Upgrade::Popularity => !self.popularity_evolved,
            Upgrade::Power => !self.power_evolved,
            Upgrade::Card => !self.card_evolved,
            Upgrade::Move => !self.move_evolved,
            Upgrade::Tax => !self.tax_evolved,
            Upgrade::Produce => !self.produce_evolved,
        }
    }

    pub(crate) fn can_upgrade_secondary(&self, secondary: SecondaryAction) -> bool {
        match secondary {
            SecondaryAction::Upgrade => self.upgrade_evolution_cost > 0,
            SecondaryAction::Deploy => self.deploy_evolution_cost > 0,
            SecondaryAction::Build => self.build_evolution_cost > 0,
            SecondaryAction::Enlist => self.enlist_evolution_cost > 0,
        }
    }

    pub(crate) fn get_upgrade_cost(&self, secondary: SecondaryAction) -> i32 {
        match secondary {
            SecondaryAction::Upgrade => self.upgrade_base_cost + self.upgrade_evolution_cost,
            SecondaryAction::Deploy => self.deploy_base_cost + self.deploy_evolution_cost,
            SecondaryAction::Build => self.build_base_cost + self.build_evolution_cost,
            SecondaryAction::Enlist => self.enlist_base_cost + self.enlist_evolution_cost,
        }
    }

    pub(crate) fn get_upgrade_coins(&self, secondary: SecondaryAction) -> i32 {
        match secondary {
            SecondaryAction::Upgrade => self.upgrade_coins,
            SecondaryAction::Deploy => self.deploy_coins,
            SecondaryAction::Build => self.build_coins,
            SecondaryAction::Enlist => self.enlist_coins,
        }
    }
}
