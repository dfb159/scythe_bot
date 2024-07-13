use crate::game::turnmask::Tile;

pub mod faction;
pub mod player_mat;

#[derive(Debug)]
pub(crate) struct Player<'a> {
    pub(crate) name: &'a str,
    pub(crate) bonus_starting_coins: i32,
    pub(crate) bonus_starting_power: i32,
    pub(crate) bonus_starting_popularity: i32,
}

#[derive(Debug)]
pub(crate) struct Faction<'a> {
    pub(crate) name: &'a str,
    pub(crate) starting_power: i32,
    pub(crate) starting_cards: i32,

    pub(crate) first_starting_field: Tile,
    pub(crate) second_starting_field: Tile,
}

#[derive(Debug)]
pub(crate) struct PlayerMat<'a> {
    pub(crate) name: &'a str,
    pub(crate) starting_index: i32,

    pub(crate) starting_coins: i32,
    pub(crate) starting_popularity: i32,

    pub(crate) move_secondary: SecondaryAction, // for move and tax primary actions
    pub(crate) trade_secondary: SecondaryAction, // for trade and promote primary actions
    pub(crate) produce_secondary: SecondaryAction, // for produce primary action
    pub(crate) bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    pub(crate) upgrade_cost: i32,
    pub(crate) upgrade_evolutions: i32,
    pub(crate) upgrade_coins: i32,
    pub(crate) deploy_cost: i32,
    pub(crate) deploy_evolutions: i32,
    pub(crate) deploy_coins: i32,
    pub(crate) build_cost: i32,
    pub(crate) build_evolutions: i32,
    pub(crate) build_coins: i32,
    pub(crate) enlist_cost: i32,
    pub(crate) enlist_evolutions: i32,
    pub(crate) enlist_coins: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PrimaryAction {
    Move,
    Tax,
    Trade,
    Promote,
    Bolster,
    Enforce,
    Produce,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum SecondaryAction {
    Upgrade,
    Deploy,
    Build,
    Enlist,
}
