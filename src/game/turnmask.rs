#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Tile {
    Woods,
    Tundra,
    Mountain,
    Farm,
    Village,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Resource {
    Wood,
    Metal,
    Oil,
    Food,
}

pub(crate) type Movement = (Tile, Tile);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Move {
    Move1(Movement),
    Move2(Movement, Movement),
    Move3(Movement, Movement, Movement),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Produce {
    Produce1(Tile),
    Produce2(Tile, Tile),
    Produce3(Tile, Tile, Tile),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Primary {
    Move(Move),
    Tax,
    Trade(Resource, Resource),
    Promote,
    Bolster,
    Enforce,
    Produce(Produce),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Mech {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PrimaryUpgrade {
    Move,
    Tax,
    Promote,
    Produce,
    Bolster,
    Enforce,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum SecondaryUpgrade {
    Upgrade,
    Deploy,
    Build,
    Enlist,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Building {
    Armory,
    Monument,
    Tunnel,
    Mill(Tile),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Recruit {
    Popularity,
    Power,
    Card,
    Coin,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Secondary {
    Upgrade(PrimaryUpgrade, SecondaryUpgrade),
    Deploy(Mech),
    Build(Building),
    Enlist(Recruit, Recruit),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum TurnMask {
    PrimaryOnly(Primary),
    PrimaryAndSecondary(Primary, Secondary),
}
