use crate::{
    game::{board::ResourceField, buildings::Building, mechs::Mech, production::Worker, recruits::Recruit, upgrades::{PrimaryUpgrade, SecondaryUpgrade}, Resource},
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
    Produce1(Position),
    Produce2(Position, Position),
    Produce3(Position, Position, Position),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Trade {
    Trade1(TradeUnit),
    Trade2(TradeUnit, TradeUnit),
}

pub type TradeUnit = (Position, Resource, Resource);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Move { // Might trigger a combat encounter
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
pub type MechMove = (Position, u8, ResourceField);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Secondary {
    Upgrade(PrimaryUpgrade, SecondaryUpgrade),
    Deploy(Mech, Worker),
    Build(Building, Worker),
    Enlist(Recruit, Recruit),
}
