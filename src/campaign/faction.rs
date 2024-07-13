use crate::game::turnmask::Tile;

use super::Faction;

pub const RUSVIET: Faction = Faction {
    name: "Rusviet",
    starting_power: 3,
    starting_cards: 1,
    first_starting_field: Tile::Village,
    second_starting_field: Tile::Mountain,
};
