use crate::game::turnmask::Resource;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ResourcesState {
    pub(crate) wood: i32,
    pub(crate) metal: i32,
    pub(crate) oil: i32,
    pub(crate) food: i32,
}

impl ResourcesState {
    pub(crate) fn new() -> ResourcesState {
        ResourcesState {
            wood: 0,
            metal: 0,
            oil: 0,
            food: 0,
        }
    }

    pub(crate) fn total(&self) -> i32 {
        self.wood + self.metal + self.oil + self.food
    }

    pub(crate) fn add(&mut self, resource: Resource, amount: i32) {
        match resource {
            Resource::Wood => {
                self.wood += amount;
            }
            Resource::Metal => {
                self.metal += amount;
            }
            Resource::Oil => {
                self.oil += amount;
            }
            Resource::Food => {
                self.food += amount;
            }
        }
    }

    pub(crate) fn get(&self, resource: Resource) -> i32 {
        match resource {
            Resource::Wood => self.wood,
            Resource::Metal => self.metal,
            Resource::Oil => self.oil,
            Resource::Food => self.food,
        }
    }
}
