use crate::game::Tile;

pub mod faction;
pub mod player_mat;
pub mod board;

#[derive(Debug, Clone)]
pub struct Player<'a> {
    pub name: &'a str,
    pub bonus_starting_coins: u32,
    pub bonus_starting_power: u8,
    pub bonus_starting_popularity: u8,
}

#[derive(Debug, Clone)]
pub struct Faction<'a> {
    pub name: &'a str,
    pub starting_power: u8,
    pub starting_cards: u8,

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


#[derive(Debug, Clone)]
pub struct PlayerMat<'a> {
    pub name: &'a str,
    pub starting_index: u8,

    pub starting_coins: u32,
    pub starting_popularity: u8,

    pub move_secondary: SecondaryAction, // for move and tax primary actions
    pub trade_secondary: SecondaryAction, // for trade and promote primary actions
    pub produce_secondary: SecondaryAction, // for produce primary action
    pub bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    pub upgrade_cost: u8,
    pub upgrade_evolutions: u8,
    pub upgrade_coins: u32,
    pub deploy_cost: u8,
    pub deploy_evolutions: u8,
    pub deploy_coins: u32,
    pub build_cost: u8,
    pub build_evolutions: u8,
    pub build_coins: u32,
    pub enlist_cost: u8,
    pub enlist_evolutions: u8,
    pub enlist_coins: u32,
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
    pub fields: [FieldTemplate; F],
    pub rivers: [(Position,Position); R],
    pub starting_locations: [HomeTemplate; P],
}

#[derive(Debug)]
pub struct FieldTemplate {
    pub position: Position,
    pub tile: Tile,
    pub tunnelable: bool,
    pub explorer_token: bool,
}

#[derive(Debug)]
pub struct HomeTemplate {
    pub position: Position,
    pub start1: Position,
    pub start2: Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(i8, i8);