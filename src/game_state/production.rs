use std::cmp::min;

use crate::game::turnmask::Tile;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProductionState {
    wood: i32,
    metal: i32,
    oil: i32,
    food: i32,
    population: i32,

    pub(crate) star: bool,
}

impl ProductionState {
    pub(crate) fn new(starting_fields: Vec<Tile>) -> ProductionState {
        let mut state = ProductionState {
            wood: 0,
            metal: 0,
            oil: 0,
            food: 0,
            population: 0,
            star: false,
        };

        starting_fields.iter().for_each(|tile| {
            state.add(tile, 1);
        });

        state
    }

    pub(crate) fn add(&mut self, tile: &Tile, amount: i32) {
        let reduced = min(amount, 8 - self.total());

        match tile {
            Tile::Woods => self.wood += reduced,
            Tile::Mountain => self.metal += reduced,
            Tile::Tundra => self.oil += reduced,
            Tile::Farm => self.food += reduced,
            Tile::Village => self.population += reduced,
        }

        if self.total() >= 8 {
            self.star = true;
        }
    }

    pub(crate) fn get(&self, tile: &Tile) -> i32 {
        match tile {
            Tile::Woods => self.wood,
            Tile::Mountain => self.metal,
            Tile::Tundra => self.oil,
            Tile::Farm => self.food,
            Tile::Village => self.population,
        }
    }

    pub(crate) fn total(&self) -> i32 {
        self.wood + self.metal + self.oil + self.food + self.population
    }
}
