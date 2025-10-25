use crate::game::turnmask::Tile;

pub mod faction;
pub mod player_mat;
pub mod board;

#[derive(Debug)]
pub struct Player<'a> {
    pub name: &'a str,
    pub bonus_starting_coins: i32,
    pub bonus_starting_power: i32,
    pub bonus_starting_popularity: i32,
}

#[derive(Debug)]
pub struct Faction<'a> {
    pub name: &'a str,
    pub starting_power: i32,
    pub starting_cards: i32,

    pub riverwalk_tile1: Tile,
    pub riverwalk_tile2: Tile,
    pub mobility_power: MobilityPower,
    pub combat_power: CombatPower,
    pub faction_ability: FactionAbility,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MobilityPower {
    Underpass,
    Township,
    Seaworthy,
    Wayfare,
    Submerge,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CombatPower {
    Disarm,
    PeoplesArmy,
    Artillery,
    Scout,
    Camaraderie,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FactionAbility {
    Relentless,
    Coercion,
    Swim,
    Meander,
    Dominate,
}


#[derive(Debug)]
pub struct PlayerMat<'a> {
    pub name: &'a str,
    pub starting_index: i32,

    pub starting_coins: i32,
    pub starting_popularity: i32,

    pub move_secondary: SecondaryAction, // for move and tax primary actions
    pub trade_secondary: SecondaryAction, // for trade and promote primary actions
    pub produce_secondary: SecondaryAction, // for produce primary action
    pub bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    pub upgrade_cost: i32,
    pub upgrade_evolutions: i32,
    pub upgrade_coins: i32,
    pub deploy_cost: i32,
    pub deploy_evolutions: i32,
    pub deploy_coins: i32,
    pub build_cost: i32,
    pub build_evolutions: i32,
    pub build_coins: i32,
    pub enlist_cost: i32,
    pub enlist_evolutions: i32,
    pub enlist_coins: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PrimaryAction {
    Move,
    Tax,
    Trade,
    Promote,
    Bolster,
    Enforce,
    Produce,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SecondaryAction {
    Upgrade,
    Deploy,
    Build,
    Enlist,
}

#[derive(Debug)]
pub struct BoardTemplate<const F: usize, const R: usize, const P: usize> {
    fields: [FieldTemplate; F],
    rivers: [(Position,Position); R],
    starting_locations: [Position; P],
}

#[derive(Debug)]
pub struct FieldTemplate {
    position: Position,
    tile: Tile,
    tunnelable: bool,
    explorer_token: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(i8, i8);