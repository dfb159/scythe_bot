use crate::template::{PlayerMat, SecondaryAction};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PrimaryUpgrade {
    Move,
    Tax,
    Promote,
    Produce,
    Bolster,
    Enforce,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SecondaryUpgrade {
    Upgrade,
    Deploy,
    Build,
    Enlist,
}

#[derive(Debug, Clone, Copy)]
pub struct UpgradesState {
    pub popularity_evolved: bool,
    pub power_evolved: bool,
    pub card_evolved: bool,
    pub move_evolved: bool,
    pub tax_evolved: bool,
    pub produce_evolved: bool,

    pub upgrade_base_cost: u8,
    pub upgrade_evolution_cost: u8,
    pub upgrade_coins: u32,
    pub deploy_base_cost: u8,
    pub deploy_evolution_cost: u8,
    pub deploy_coins: u32,
    pub build_base_cost: u8,
    pub build_evolution_cost: u8,
    pub build_coins: u32,
    pub enlist_base_cost: u8,
    pub enlist_evolution_cost: u8,
    pub enlist_coins: u32,

    pub star: bool,
}

impl UpgradesState {
    pub fn new(mat: &PlayerMat) -> UpgradesState {
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

    pub fn upgrade(&mut self, primary: PrimaryUpgrade, secondary: SecondaryUpgrade) {
        match primary {
            PrimaryUpgrade::Promote => {
                self.popularity_evolved = true;
            }
            PrimaryUpgrade::Bolster => {
                self.power_evolved = true;
            }
            PrimaryUpgrade::Enforce => {
                self.card_evolved = true;
            }
            PrimaryUpgrade::Move => {
                self.move_evolved = true;
            }
            PrimaryUpgrade::Tax => {
                self.tax_evolved = true;
            }
            PrimaryUpgrade::Produce => {
                self.produce_evolved = true;
            }
        }
        match secondary {
            SecondaryUpgrade::Upgrade => {
                self.upgrade_evolution_cost = self.upgrade_evolution_cost.saturating_sub(1);
            }
            SecondaryUpgrade::Deploy => {
                self.deploy_evolution_cost = self.deploy_evolution_cost.saturating_sub(1);
            }
            SecondaryUpgrade::Build => {
                self.build_evolution_cost = self.build_evolution_cost.saturating_sub(1);
            }
            SecondaryUpgrade::Enlist => {
                self.enlist_evolution_cost = self.enlist_evolution_cost.saturating_sub(1);
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

    pub fn can_upgrade(&self, primary: &PrimaryUpgrade, secondary: &SecondaryUpgrade) -> bool {
        self.can_upgrade_primary(primary) && self.can_upgrade_secondary(secondary)
    }

    pub fn can_upgrade_primary(&self, primary: &PrimaryUpgrade) -> bool {
        match primary {
            PrimaryUpgrade::Promote => !self.popularity_evolved,
            PrimaryUpgrade::Bolster => !self.power_evolved,
            PrimaryUpgrade::Enforce => !self.card_evolved,
            PrimaryUpgrade::Move => !self.move_evolved,
            PrimaryUpgrade::Tax => !self.tax_evolved,
            PrimaryUpgrade::Produce => !self.produce_evolved,
        }
    }

    pub fn can_upgrade_secondary(&self, secondary: &SecondaryUpgrade) -> bool {
        match secondary {
            SecondaryUpgrade::Upgrade => self.upgrade_evolution_cost > 0,
            SecondaryUpgrade::Deploy => self.deploy_evolution_cost > 0,
            SecondaryUpgrade::Build => self.build_evolution_cost > 0,
            SecondaryUpgrade::Enlist => self.enlist_evolution_cost > 0,
        }
    }

    pub fn get_upgrade_cost(&self, secondary: &SecondaryAction) -> u8 {
        match secondary {
            SecondaryAction::Upgrade => self
                .upgrade_base_cost
                .saturating_add(self.upgrade_evolution_cost),
            SecondaryAction::Deploy => self
                .deploy_base_cost
                .saturating_add(self.deploy_evolution_cost),
            SecondaryAction::Build => self
                .build_base_cost
                .saturating_add(self.build_evolution_cost),
            SecondaryAction::Enlist => self
                .enlist_base_cost
                .saturating_add(self.enlist_evolution_cost),
        }
    }

    pub fn get_upgrade_coins(&self, secondary: &SecondaryUpgrade) -> u32 {
        match secondary {
            SecondaryUpgrade::Upgrade => self.upgrade_coins,
            SecondaryUpgrade::Deploy => self.deploy_coins,
            SecondaryUpgrade::Build => self.build_coins,
            SecondaryUpgrade::Enlist => self.enlist_coins,
        }
    }
}
