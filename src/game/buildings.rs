use std::rc::Rc;

use crate::game::board::Field;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Building {
    Armory,
    Monument,
    Tunnel,
    Mill,
}

#[derive(Debug, Clone)]
pub struct BuildingsState {
    pub mine: Option<Rc<Field>>,
    pub mill: Option<Rc<Field>>,
    pub armory: Option<Rc<Field>>,
    pub monument: Option<Rc<Field>>,

    pub star: bool,
}

impl BuildingsState {
    pub fn new() -> BuildingsState {
        BuildingsState {
            star: false,
            mine: Option::None,
            mill: Option::None,
            armory: Option::None,
            monument: Option::None,
        }
    }

    pub fn built(&mut self, building: Building, location: &Rc<Field>) {
        match building {
            Building::Tunnel => {
                self.mine = Option::Some(location.clone());
            }
            Building::Mill => {
                self.mill = Option::Some(location.clone());
            }
            Building::Armory => {
                self.armory = Option::Some(location.clone());
            }
            Building::Monument => {
                self.monument = Option::Some(location.clone());
            }
        }
        if self.mine.is_some()
            && self.mill.is_some()
            && self.armory.is_some()
            && self.monument.is_some()
        {
            self.star = true;
        }
    }

    pub fn can_build(&self, building: Building) -> bool {
        match building {
            Building::Tunnel => self.mine.is_none(),
            Building::Mill => self.mill.is_none(),
            Building::Armory => self.armory.is_none(),
            Building::Monument => self.monument.is_none(),
        }
    }

    pub fn is_build(&self, building: Building) -> bool {
        !self.can_build(building)
    }
}
