use super::PlayerMat;
use super::SecondaryAction;

pub const INDUSTRIAL: PlayerMat = PlayerMat {
    name: "Industrial".to_string(),
    starting_index: 1,
    starting_coins: 4,
    starting_popularity: 2,
    move_secondary: SecondaryAction::Build,
    trade_secondary: SecondaryAction::Enlist,
    produce_secondary: SecondaryAction::Deploy,
    bolster_secondary: SecondaryAction::Upgrade,
    upgrade_cost: 2,
    upgrade_evolutions: 1,
    upgrade_coins: 3,
    deploy_cost: 1,
    deploy_evolutions: 2,
    deploy_coins: 2,
    build_cost: 2,
    build_evolutions: 1,
    build_coins: 1,
    enlist_cost: 2,
    enlist_evolutions: 2,
    enlist_coins: 0,
};
