use std::rc::Rc;

use crate::{
    game::{Resource, board::Field, game::Game, player::PlayerState},
    turn::mask::{Move, Primary, Trade, TradeUnit, UnitMovement},
};

pub type Reason = Option<&'static str>;

pub fn check_primary(game: &Game, primary: &Primary) -> Reason {
    match primary {
        Primary::Move(Move::Move1(m)) => check_movement(&game, vec![m]),
        Primary::Move(Move::Move2(m1, m2)) => check_movement(&game, vec![m1, m2]),
        Primary::Move(Move::Move3(m1, m2, m3)) => {
            match !game.get_active_player().upgrades.move_evolved {
                true => Some("Move3 is not evolved"),
                false => check_movement(&game, vec![m1, m2, m3]),
            }
        }
        Primary::Tax => None,
        Primary::Trade(trade) => match game.get_active_player().coins >= 1 {
            true => match trade {
                Trade::Trade1(u) => check_trade1(game, u),
                Trade::Trade2(u1, u2) => check_trade2(game, u1, u2),
            },
            false => Some("Not enough coins for trading"),
        },
        Primary::Promote => match game.get_active_player().coins >= 1 {
            true => None,
            false => Some("Not enough coins"),
        },
        Primary::Bolster => match game.get_active_player().coins >= 1 {
            true => None,
            false => Some("Not enough coins"),
        },
        Primary::Enforce => match game.get_active_player().coins >= 1 {
            true => None,
            false => Some("Not enough coins"),
        },
        Primary::Produce(Produce1(tile1)) => {
            game.can_produce() && check_movement(&produce_game(), vec![tile1])
        }
        Primary::Produce(Produce2(tile1, tile2)) => {
            game.can_produce() && check_movement(&produce_game(), vec![tile1, tile2])
        }
        Primary::Produce(Produce3(tile1, tile2, tile3)) => {
            game.can_produce()
                && game.upgrades.produce_evolved
                && check_movement(&produce_game(), vec![tile1, tile2, tile3])
        }
    }
}

/// Check if the the tiles are valid and do not exceed the production game
fn check_movement(game: &Game, movement: Vec<&UnitMovement>) -> Reason {
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

pub fn check_secondary_cost(game: &Playergame, secondary: &SecondaryAction) -> bool {
    (match secondary {
        SecondaryAction::Upgrade => game.resources.oil,
        SecondaryAction::Deploy => game.resources.metal,
        SecondaryAction::Build => game.resources.wood,
        SecondaryAction::Enlist => game.resources.food,
    }) >= game.upgrades.get_upgrade_cost(secondary)
}
