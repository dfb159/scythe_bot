use std::{iter::zip, rc::Rc};

use crate::{game::board::Field, turn::mask::WorkerMask};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Worker {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

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

    pub fn amount(&self, field: &Rc<Field>) -> u8 {
        self.workers.iter().fold(0, |acc, w| {
            acc + match w {
                Some(t) if Rc::ptr_eq(field, &t) => 1,
                _ => 0,
            }
        })
    }

    pub fn get(&self, worker: Worker) -> Option<&WorkerEntity> {
        self.workers[worker as usize].as_ref()
    }

    pub fn get_deployed(&self) -> WorkerMask {
        zip(self.workers.iter(), [WorkerMask::all()]).fold(
            WorkerMask::empty(),
            |mask, (worker, m)| match worker {
                Some(_) => mask | m, // if the worker is deployed
                _ => mask,
            },
        )
    }

    pub fn at(&self, field: &Rc<Field>) -> WorkerMask {
        zip(self.workers.iter(), [WorkerMask::all()]).fold(
            WorkerMask::empty(),
            |mask, (worker, m)| match worker {
                Some(f) if Rc::ptr_eq(field, &f) => mask | m, // if the worker is deployed and at this field
                _ => mask,
            },
        )
    }
}
