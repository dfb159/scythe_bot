use std::rc::Rc;

use crate::game::board::Field;

pub type WorkerEntity = Rc<Field>;

#[derive(Debug, Clone)]
pub struct ProductionState {
    pub workers: [Option<WorkerEntity>; 8],
    pub deployed_workers: usize,
    pub star: bool,
}

impl ProductionState {
    pub fn new() -> ProductionState {
        let state = ProductionState {
            workers: [const { None }; 8],
            deployed_workers: 0,
            star: false,
        };

        state
    }

    pub fn deploy(&mut self, tile: &Rc<Field>) {
        if self.deployed_workers < self.workers.len() {
            self.workers[self.deployed_workers] = Some(tile.clone());
        }

        if self.deployed_workers >= self.workers.len() {
            self.star = true;
        }
    }

    pub fn get(&self, tile: &Rc<Field>) -> u8 {
        self.workers.iter().fold(0, |acc, w| {
            acc + match w {
                Some(t) if Rc::ptr_eq(tile, &t) => 1,
                _ => 0,
            }
        })
    }
}
