pub(crate) mod human;
pub(crate) mod random;

use crate::{
    campaign::{PrimaryAction, SecondaryAction},
    game_state::{
        buildings::Building, mechs::Mech, recruits::Recruit, resources::Resource,
        upgrades::Upgrade, PlayerState,
    },
};

pub(crate) trait Agent {
    fn prepare(&mut self, state: &PlayerState);
    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction;
    fn choose_trade(&self, state: &PlayerState) -> Resource;
    fn choose_produce(&self, state: &PlayerState) -> Resource;
    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)>;
    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)>;
    fn deploy(&self, state: &PlayerState) -> Option<Mech>;
    fn build(&self, state: &PlayerState) -> Option<Building>;
    fn choose_mill_location(&self, state: &PlayerState) -> Resource;
    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)>;
}
