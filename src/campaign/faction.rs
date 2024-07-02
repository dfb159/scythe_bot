use super::Faction;
use super::Resource;

pub const RUSVIET: Faction = Faction {
    name: "Rusviet",
    starting_power: 3,
    starting_cards: 1,
    first_starting_field: Resource::People,
    second_starting_field: Resource::Metal,
};
