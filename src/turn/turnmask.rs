pub type Movement = (Tile, Tile);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Move {
    Move1(Movement),
    Move2(Movement, Movement),
    Move3(Movement, Movement, Movement),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Produce {
    Produce1(Tile),
    Produce2(Tile, Tile),
    Produce3(Tile, Tile, Tile),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Primary {
    Move(Move),
    Tax,
    Trade(Resource, Resource),
    Promote,
    Bolster,
    Enforce,
    Produce(Produce),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Secondary {
    Upgrade(PrimaryUpgrade, SecondaryUpgrade),
    Deploy(Mech),
    Build(Building),
    Enlist(Recruit, Recruit),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TurnMask {
    PrimaryOnly(Primary),
    PrimaryAndSecondary(Primary, Secondary),
}
