use bitflags::bitflags;

use crate::{
    game::{
        Resource,
        board::ResourceField,
        buildings::Building,
        mechs::Mech,
        production::Worker,
        recruits::Recruit,
        upgrades::{PrimaryUpgrade, SecondaryUpgrade},
    },
    template::Position,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TurnMask {
    PrimaryOnly(Primary),
    PrimaryAndSecondary(Primary, Secondary),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Primary {
    Bolster,
    Enforce,
    Produce(Produce),
    Move(Move),
    Tax,
    Trade(Trade),
    Promote,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Produce {
    Produce1(Worker),
    Produce2(Worker, Worker),
    Produce3(Worker, Worker, Worker),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Trade {
    Trade1(TradeUnit),
    Trade2(TradeUnit, TradeUnit),
}

pub type TradeUnit = (UnitPosition, Resource, Resource);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Move {
    // Might trigger a combat encounter
    Move1(UnitMovement),
    Move2(UnitMovement, UnitMovement),
    Move3(UnitMovement, UnitMovement, UnitMovement),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnitMovement {
    Character(Movement<NormalMove>), // Might trigger an encounter
    Worker(Worker, Movement<NormalMove>),
    Mech(Mech, Movement<MechMove>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Movement<Pos> {
    Single(Pos),
    Double(Pos, Pos),
}

pub type NormalMove = (Position, ResourceField);
pub type MechMove = (Position, WorkerMask, ResourceField);

bitflags! {
    // Attributes can be applied to flags types
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WorkerMask: u8 {
        const w1 = 0b00000001;
        const w2 = 0b00000010;
        const w3 = 0b00000100;
        const w4 = 0b00001000;
        const w5 = 0b00010000;
        const w6 = 0b00100000;
        const w7 = 0b01000000;
        const w8 = 0b10000000;

        // The source may set any bits
        const _ = !0;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Secondary {
    Upgrade(PrimaryUpgrade, SecondaryUpgrade, ResourceCost),
    Deploy(Mech, Worker, ResourceCost),
    Build(Building, Worker, ResourceCost),
    Enlist(Recruit, Recruit, ResourceCost),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceCost {
    One(UnitPosition),
    Two(UnitPosition, UnitPosition),
    Three(UnitPosition, UnitPosition, UnitPosition),
    Four(UnitPosition, UnitPosition, UnitPosition, UnitPosition),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnitPosition {
    Character,
    Worker(Worker),
    Mech(Mech),
    Building(Building),
}
