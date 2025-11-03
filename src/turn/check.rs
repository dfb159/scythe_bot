use std::rc::Rc;

use crate::{
    game::{
        Resource, Tile,
        board::{Field, ResourceField},
        game::Game,
        mechs::Mech,
        player::PlayerState,
        production::Worker,
    },
    template::Position,
    turn::mask::{
        MechMask, Move, Movement, Primary, Produce, Trade, TradeUnit, UnitMovement, WorkerMask,
    },
};

pub type Reason = Option<&'static str>;

pub fn check_primary(game: &Game, primary: &Primary) -> Reason {
    let player = game.get_active_player();
    let mut history = History::new();
    match primary {
        Primary::Move(Move::Move1(m)) => check_move(game, player, &[m], &mut history),
        Primary::Move(Move::Move2(m1, m2)) => check_move(game, player, &[m1, m2], &mut history),
        Primary::Move(Move::Move3(m1, m2, m3)) => (!player.upgrades.move_evolved)
            .then_some("Move3 is not evolved")
            .or_else(|| check_move(game, player, &[m1, m2, m3], &mut history)),
        Primary::Tax => None,
        Primary::Trade(trade) => (player.coins < 1)
            .then_some("Not enough coins to trade")
            .or_else(|| match trade {
                Trade::Trade1(u) => check_trade1(game, u),
                Trade::Trade2(u1, u2) => check_trade2(game, u1, u2),
            }),
        Primary::Promote => (player.coins < 1).then_some("Not enough coins to promote"),
        Primary::Bolster => (player.coins < 1).then_some("Not enough coins to bolster"),
        Primary::Enforce => (player.coins < 1).then_some("Not enough coins to enforce"),
        Primary::Produce(Produce::Produce1(tile1)) => check_produce(player, &[tile1]),
        Primary::Produce(Produce::Produce2(tile1, tile2)) => check_produce(player, &[tile1, tile2]),
        Primary::Produce(Produce::Produce3(tile1, tile2, tile3)) => {
            (!player.upgrades.produce_evolved)
                .then_some("Produce3 is not evolved")
                .or_else(|| check_produce(player, &[tile1, tile2, tile3]))
        }
    }
}

pub fn check_move(
    game: &Game,
    player: &Rc<PlayerState>,
    movement: &[&UnitMovement],
    history: &mut History,
) -> Reason {
    movement.iter().fold(None, |acc: Reason, &mov| {
        acc.or_else(|| match mov {
            UnitMovement::Character(m) => {
                check_character_move(game, &player.character.location, m, history)
            }
            UnitMovement::Worker(worker, m) => {
                check_worker_move(game, &player, *worker, m, history)
            }
            UnitMovement::Mech(mech, m) => check_mech_move(game, &player, *mech, m, history),
        })
    })
}

type WorkerHistory = (Rc<Field>, Rc<Field>, WorkerMask);
type ResourceHistory = (Rc<Field>, Rc<Field>, ResourceField);

#[derive(Debug, Clone, PartialEq, Eq)]
struct History {
    worker: Vec<WorkerHistory>,
    resource: Vec<ResourceHistory>,

    character_moved: bool,
    worker_moved: WorkerMask,
    mech_moved: MechMask,
}

impl History {
    pub fn new() -> Self {
        History {
            worker: Vec::new(),
            resource: Vec::new(),
            character_moved: false,
            worker_moved: WorkerMask::empty(),
            mech_moved: MechMask::empty(),
        }
    }
}

/// Are all the required resources at the source field after moves?
pub fn check_resources(
    from: &Rc<Field>,
    to: &Rc<Field>,
    amount: ResourceField,
    history: &mut History,
) -> Reason {
    let mut available = from.resources;
    for (a, b, amt) in history.resource.iter() {
        if Rc::ptr_eq(a, b) {
            continue;
        }
        if Rc::ptr_eq(a, from) {
            match available.checked_sub(*amt) {
                Some(sum) => available = sum,
                None => return Some("Resource history is negative"),
            };
        }
        if Rc::ptr_eq(b, from) {
            available = available + *amt;
        }
    }

    if available >= amount {
        history.resource.push((from.clone(), to.clone(), amount));
        None
    } else {
        Some("Not enough resources to take on this move")
    }
}

/// Are all the required workers of the mask at the source field?
pub fn check_carry_workers(
    player: &Rc<PlayerState>,
    from: &Rc<Field>,
    to: &Rc<Field>,
    workers: WorkerMask,
    history: &mut History,
) -> Reason {
    let mut stationed = player.production.at(from);
    for (a, b, w) in history.worker.iter() {
        if Rc::ptr_eq(a, b) {
            continue;
        }
        if Rc::ptr_eq(a, from) {
            if !stationed.contains(*w) {
                return Some("Required workers are not stationed at the field");
            }
            stationed = stationed.difference(*w)
        }
        if Rc::ptr_eq(b, from) {
            if stationed.intersects(*w) {
                return Some("Required workers are already stationed at the field");
            }
            stationed = stationed.union(*w)
        }
    }

    if !stationed.contains(workers) {
        Some("Source field does not have all the required workers stationed")
    } else if stationed.intersects(workers) {
        Some("Target field already has some required workers stationed")
    } else {
        history.worker.push((from.clone(), to.clone(), workers));
        None
    }
}

pub fn check_character_move(
    game: &Game,
    from: &Rc<Field>,
    mov: &Movement<(Position, ResourceField)>,
    history: &mut History,
) -> Reason {
    match mov {
        super::mask::Movement::Single((pos, res)) => match game.board.get_field(pos) {
            Some(to) => check_character_movement(from, to, true, history)
                .or_else(|| check_resources(from, to, *res, history)),
            None => Some("Target position is not a valid field"),
        },
        super::mask::Movement::Double((p1, r1), (p2, r2)) => {
            match (game.board.get_field(p1), game.board.get_field(p2)) {
                (None, None) => Some("Target positions are not valid fields"),
                (None, Some(_)) => Some("First target positions is not a valid field"),
                (Some(_), None) => Some("Second target positions is not a valid field"),
                (Some(t1), Some(t2)) => check_character_movement(from, t1, true, history)
                    .or_else(|| check_resources(from, t2, *r1, history))
                    .or_else(|| check_character_movement(t1, t2, false, history))
                    .or_else(|| check_resources(t1, t2, *r2, history)),
            }
        }
    }
}

pub fn check_worker_move(
    game: &Game,
    player: &Rc<PlayerState>,
    worker: Worker,
    mov: &Movement<(Position, ResourceField)>,
    history: &mut History,
) -> Reason {
    match player.production.get(worker) {
        Some(from) => match mov {
            super::mask::Movement::Single((pos, res)) => match game.board.get_field(pos) {
                Some(to) => check_worker_movement(from, to, worker, true, history)
                    .or_else(|| check_resources(from, to, *res, history)),
                None => Some("Target position is not a valid field"),
            },
            super::mask::Movement::Double((p1, r1), (p2, r2)) => {
                match (game.board.get_field(p1), game.board.get_field(p2)) {
                    (None, None) => Some("Target positions are not valid fields"),
                    (None, Some(_)) => Some("First target positions is not a valid field"),
                    (Some(_), None) => Some("Second target positions is not a valid field"),
                    (Some(t1), Some(t2)) => check_worker_movement(from, t1, worker, true, history)
                        .or_else(|| check_resources(from, t1, *r1, history))
                        .or_else(|| check_worker_movement(t1, t2, worker, false, history))
                        .or_else(|| check_resources(t1, t2, *r2, history)),
                }
            }
        },
        None => Some("Worker is not deployed"),
    }
}

fn check_mech_move(
    game: &Game,
    player: &Rc<PlayerState>,
    mech: Mech,
    mov: &Movement<(Position, WorkerMask, ResourceField)>,
    history: &mut History,
) -> Reason {
    match player.mechs.get(mech) {
        Some(from) => match mov {
            super::mask::Movement::Single((pos, workers, res)) => match game.board.get_field(pos) {
                Some(to) => check_mech_movement(from, to, mech, true, history)
                    .or_else(|| check_resources(from, to, *res, history))
                    .or_else(|| check_carry_workers(&player, from, to, *workers, history)),
                None => Some("Target position is not a valid field"),
            },
            super::mask::Movement::Double((p1, w1, r1), (p2, w2, r2)) => {
                match (game.board.get_field(p1), game.board.get_field(p2)) {
                    (None, None) => Some("Target positions are not valid fields"),
                    (None, Some(_)) => Some("First target positions is not a valid field"),
                    (Some(_), None) => Some("Second target positions is not a valid field"),
                    (Some(t1), Some(t2)) => check_mech_movement(from, t1, mech, true, history)
                        .or_else(|| check_resources(from, t1, *r1, history))
                        .or_else(|| check_carry_workers(player, from, t1, *w1, history))
                        .or_else(|| check_mech_movement(t1, t2, mech, false, history))
                        .or_else(|| check_resources(t1, t2, *r2, history))
                        .or_else(|| check_carry_workers(player, t1, t2, *w2, history)),
                }
            }
        },
        None => Some("Mech is not deployed"),
    }
}

/// Can the character move from this field to that field in a single move?
pub fn check_character_movement(
    from: &Rc<Field>,
    to: &Rc<Field>,
    check_already_moved: bool,
    history: &mut History,
) -> Reason {
    if check_already_moved && history.character_moved {
        return Some("Cannot move the character multiple times in one turn");
    }
    // cannot move from a field of conflict
    // might move from lake to lake
    // might move from/to lake
    // might move from/to home
    // might move through tunnels
    // might move over rivers
    // can move to the field very next to the source

    history.character_moved = true;
    None
}

/// Can the worker move from this field to that field in a single move?
pub fn check_worker_movement(
    from: &Rc<Field>,
    to: &Rc<Field>,
    worker: Worker,
    check_already_moved: bool,
    history: &mut History,
) -> Reason {
    if check_already_moved && history.worker_moved.contains_worker(worker) {
        return Some("Cannot move the same worker multiple times in one turn");
    }

    // cannot move from/to a field of conflict
    // might move from lake to lake
    // might move from/to lake
    // might move from/to home
    // might move through tunnels
    // might move over rivers
    // can move to the field very next to the source

    let mask = WorkerMask::get_worker(worker);
    history.worker.push((from.clone(), to.clone(), mask));
    history.worker_moved = history.worker_moved | mask;
    None
}

/// Can the mech move from this field to that field in a single move?
pub fn check_mech_movement(
    from: &Rc<Field>,
    to: &Rc<Field>,
    mech: Mech,
    check_already_moved: bool,
    history: &mut History,
) -> Reason {
    if check_already_moved && history.mech_moved.contains_mech(mech) {
        return Some("Cannot move the same mech multiple times in one turn");
    }

    // cannot move from/to a field of conflict
    // might move from lake to lake
    // might move from/to lake
    // might move from/to home
    // might move through tunnels
    // might move over rivers
    // can move to the field very next to the source

    let mask = MechMask::get_mech(mech);
    history.mech_moved = history.mech_moved | mask;
    None
}

fn check_movement(game: &Game, movement: &UnitMovement) -> Reason {
    // no unit can walk multiple times
    // if a mech carries workers, they need to be added to the target field
    // if a unit carries resources, they need to be added to the target field
    // movement appears in the appearance in the vector
    // cannot move from a field of conflict

    None
}

pub fn check_trade1(game: &Game, (unit, from, _): &TradeUnit) -> Reason {
    let player = game.get_active_player();
    let field = player.get_unit_field(unit);
    match field {
        Some(f) => check_field_for_trade(game, &player, f, from, 1),
        None => Some("Unit is not deployed"),
    }
}

pub fn check_trade2(game: &Game, (u1, r1, _): &TradeUnit, (u2, r2, _): &TradeUnit) -> Reason {
    let player = game.get_active_player();
    let field1 = player.get_unit_field(u1);
    let field2 = player.get_unit_field(u2);
    match (field1, field2) {
        (None, None) => Some("Units are not deployed"),
        (None, Some(_)) => Some("Unit 1 is not deployed"),
        (Some(_), None) => Some("Unit 2 is not deployed"),
        (Some(f1), Some(f2)) if Rc::ptr_eq(f1, f2) && r1 == r2 => {
            check_field_for_trade(game, &player, f1, r1, 2)
        }
        (Some(f1), Some(f2)) => check_field_for_trade(game, &player, f1, r1, 1)
            .or(check_field_for_trade(game, &player, f2, r2, 1)),
    }
}

pub fn check_field_for_trade(
    game: &Game,
    player: &Rc<PlayerState>,
    field: &Rc<Field>,
    resource: &Resource,
    amount: u32,
) -> Reason {
    let control_player = game.get_player_control(field);
    if control_player.is_none() {
        Some("Field is not controlled")
    } else if let Some(p) = control_player
        && Rc::ptr_eq(p, player)
    {
        Some("Field is controlled by enemy")
    } else if !field.resources.has(resource, amount) {
        Some("Not enough resources to trade")
    } else {
        None
    }
}

const VALID_PRODUCTION_TILES: &[Tile] = &[
    Tile::Farm,
    Tile::Mountain,
    Tile::Tundra,
    Tile::Village,
    Tile::Woods,
];
pub fn check_produce(player: &Rc<PlayerState>, prod: &[&Worker]) -> Reason {
    let mut production: Vec<Rc<Field>> = Vec::new();
    for worker in prod {
        match player.production.get(**worker) {
            Some(field) => {
                if VALID_PRODUCTION_TILES.contains(&field.tile) {
                    return Some("Cannot produce on unproducible tiles");
                }
                if production.iter().any(|f| Rc::ptr_eq(f, field)) {
                    return Some("Cannot produce on the same tile multiple times");
                }
                production.push(field.clone());
            }
            None => return Some("Worker is not deployed and cant produce"),
        }
    }
    None
}

pub fn check_secondary_cost(game: &Playergame, secondary: &SecondaryAction) -> bool {
    (match secondary {
        SecondaryAction::Upgrade => game.resources.oil,
        SecondaryAction::Deploy => game.resources.metal,
        SecondaryAction::Build => game.resources.wood,
        SecondaryAction::Enlist => game.resources.food,
    }) >= game.upgrades.get_upgrade_cost(secondary)
}
