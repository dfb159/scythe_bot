use std::rc::Rc;

use crate::game::board::Field;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Mech {
    First,
    Second,
    Third,
    Fourth,
}

pub type MechEntity = Rc<Field>;

#[derive(Debug, Clone)]
pub struct MechsState {
    pub mechs: [Option<MechEntity>; 4],
    pub star: bool,
}

impl MechsState {
    pub fn new() -> MechsState {
        MechsState {
            star: false,
            mechs: [const { None }; 4],
        }
    }

    pub fn deploy(&mut self, mech: Mech, tile: &Rc<Field>) {
        match mech {
            Mech::First => {
                self.mechs[0] = Some(tile.clone());
            }
            Mech::Second => {
                self.mechs[1] = Some(tile.clone());
            }
            Mech::Third => {
                self.mechs[2] = Some(tile.clone());
            }
            Mech::Fourth => {
                self.mechs[3] = Some(tile.clone());
            }
        }
        if self.mechs.iter().all(|m| m.is_some()) {
            self.star = true;
        }
    }

    pub fn can_deploy(&self, mech: Mech) -> bool {
        !self.is_deployed(mech)
    }

    pub fn is_deployed(&self, mech: Mech) -> bool {
        match mech {
            Mech::First => self.mechs[0].is_some(),
            Mech::Second => self.mechs[1].is_some(),
            Mech::Third => self.mechs[2].is_some(),
            Mech::Fourth => self.mechs[3].is_some(),
        }
    }
}
