use crate::{template::{CombatPower, FactionAbility, MobilityPower}, game::Tile};

use super::Faction;

pub const SAXONY: Faction = Faction {
    name: "Saxony",
    starting_power: 1,
    starting_cards: 4,
    riverwalk_tile1: Tile::Woods,
    riverwalk_tile2: Tile::Mountain,
    mobility_power: MobilityPower::Underpass,
    combat_power: CombatPower::Disarm,
    faction_ability: FactionAbility::Dominate,
};

pub const RUSVIET: Faction = Faction {
    name: "Rusviet",
    starting_power: 3,
    starting_cards: 2,
    riverwalk_tile1: Tile::Farm,
    riverwalk_tile2: Tile::Village,
    mobility_power: MobilityPower::Township,
    combat_power: CombatPower::PeoplesArmy,
    faction_ability: FactionAbility::Relentless,
};

pub const NORDIC: Faction = Faction {
    name: "Nordic",
    starting_power: 4,
    starting_cards: 1,
    riverwalk_tile1: Tile::Woods,
    riverwalk_tile2: Tile::Mountain,
    mobility_power: MobilityPower::Seaworthy,
    combat_power: CombatPower::Artillery,
    faction_ability: FactionAbility::Swim,
};

pub const CRIMEA: Faction = Faction {
    name: "Crimea",
    starting_power: 5,
    starting_cards: 0,
    riverwalk_tile1: Tile::Farm,
    riverwalk_tile2: Tile::Tundra,
    mobility_power: MobilityPower::Wayfare,
    combat_power: CombatPower::Scout,
    faction_ability: FactionAbility::Coercion,
};

pub const POLANIA: Faction = Faction {
    name: "Polania",
    starting_power: 2,
    starting_cards: 3,
    riverwalk_tile1: Tile::Village,
    riverwalk_tile2: Tile::Mountain,
    mobility_power: MobilityPower::Submerge,
    combat_power: CombatPower::Camaraderie,
    faction_ability: FactionAbility::Meander,
};
