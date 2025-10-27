use std::rc::Rc;

use crate::game::board::Field;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Building {
    Armory,
    Monument,
    Tunnel,
    Mill,
}

pub type BuildingEntity = Rc<Field>;

#[derive(Debug, Clone)]
pub struct BuildingsState {
    pub tunnel: Option<BuildingEntity>,
    pub mill: Option<BuildingEntity>,
    pub armory: Option<BuildingEntity>,
    pub monument: Option<BuildingEntity>,

    pub star: bool,
}

impl BuildingsState {
    pub fn new() -> BuildingsState {
        BuildingsState {
            star: false,
            tunnel: Option::None,
            mill: Option::None,
            armory: Option::None,
            monument: Option::None,
        }
    }

    pub fn built(&mut self, building: Building, location: &BuildingEntity) {
        match building {
            Building::Tunnel => {
                self.tunnel = Option::Some(location.clone());
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
        if self.tunnel.is_some()
            && self.mill.is_some()
            && self.armory.is_some()
            && self.monument.is_some()
        {
            self.star = true;
        }
    }

    pub fn can_build(&self, building: Building) -> bool {
        match building {
            Building::Tunnel => self.tunnel.is_none(),
            Building::Mill => self.mill.is_none(),
            Building::Armory => self.armory.is_none(),
            Building::Monument => self.monument.is_none(),
        }
    }

    pub fn is_build(&self, building: Building) -> bool {
        !self.can_build(building)
    }

    pub fn get(&self, building: Building) -> Option<&BuildingEntity> {
        match building {
            Building::Armory => self.armory.as_ref(),
            Building::Monument => self.monument.as_ref(),
            Building::Tunnel => self.tunnel.as_ref(),
            Building::Mill => self.mill.as_ref(),
        }
    }
}
