use std::cmp::min;

use crate::Resource;

#[derive(Debug)]
pub(crate) struct ProductionState {
    pub(crate) wood: i32,
    pub(crate) metal: i32,
    pub(crate) oil: i32,
    pub(crate) food: i32,
    pub(crate) population: i32,
}

impl ProductionState {
    pub(crate) fn new(first_field: Resource, second_field: Resource) -> ProductionState {
        let mut state = ProductionState {
            wood: 0,
            metal: 0,
            oil: 0,
            food: 0,
            population: 0,
        };

        state.add(first_field, 1);
        state.add(second_field, 1);

        state
    }

    pub(crate) fn add(&mut self, resource: Resource, amount: i32) {
        let reduced = min(amount, 8 - self.total());

        match resource {
            Resource::Wood => {
                self.wood += reduced;
            }
            Resource::Metal => {
                self.metal += reduced;
            }
            Resource::Oil => {
                self.oil += reduced;
            }
            Resource::Food => {
                self.food += reduced;
            }
            Resource::People => {
                self.population += reduced;
            }
        }
    }

    pub(crate) fn get(&self, resource: Resource) -> i32 {
        match resource {
            Resource::Wood => self.wood,
            Resource::Metal => self.metal,
            Resource::Oil => self.oil,
            Resource::Food => self.food,
            Resource::People => self.population,
        }
    }

    pub(crate) fn total(&self) -> i32 {
        self.wood + self.metal + self.oil + self.food + self.population
    }
}
