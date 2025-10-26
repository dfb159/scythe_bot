pub mod board;
pub mod buildings;
pub mod game;
pub mod mechs;
pub mod military;
pub mod player;
pub mod popularity;
pub mod production;
pub mod recruits;
pub mod upgrades;
pub mod character;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tile {
    Woods,
    Tundra,
    Mountain,
    Farm,
    Village,
    Lake,
    Factory,
    Home,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Resource {
    Wood,
    Metal,
    Oil,
    Food,
}
