use std::rc::Rc;

use crate::game::board::Field;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Mech {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
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
        self.mechs[mech as usize] = Some(tile.clone());
        if self.mechs.iter().all(|m| m.is_some()) {
            self.star = true;
        }
    }

    pub fn can_deploy(&self, mech: Mech) -> bool {
        !self.is_deployed(mech)
    }

    pub fn is_deployed(&self, mech: Mech) -> bool {
        self.mechs[mech as usize].is_some()
    }

    pub fn amount(&self, field: &Rc<Field>) -> u8 {
        self.mechs.iter().fold(0, |acc, m| {
            acc + match m {
                Some(t) if Rc::ptr_eq(field, &t) => 1,
                _ => 0,
            }
        })
    }

    pub fn get(&self, mech: Mech) -> Option<&MechEntity> {
        self.mechs[mech as usize].as_ref()
    }
}
