use std::iter::repeat;

use crate::agent::Agent;
use crate::network::fcnn::{Learner, MLFunction, Predictor, FCNN};
use crate::{
    campaign::{PrimaryAction, SecondaryAction},
    game_state::{
        buildings::Building,
        mechs::Mech,
        recruits::Recruit,
        resources::Resource::{self, Food, Metal, Oil, People, Wood},
        upgrades::Upgrade,
        PlayerState,
    },
};

use ndarray::Array1;

pub(crate) struct FcnnAgent<'a> {
    // maybe extra network for option return
    primary_network: FCNN<'a>,
    trade_network: FCNN<'a>,
    produce_network: FCNN<'a>,
    move_from_network: FCNN<'a>,
    move_to_network: FCNN<'a>,
    upgrade_from_network: FCNN<'a>,
    upgrade_to_network: FCNN<'a>,
    deploy_network: FCNN<'a>,
    build_network: FCNN<'a>,
    mill_network: FCNN<'a>,
    enlist_secondary_network: FCNN<'a>,
    enlist_onetime_network: FCNN<'a>,
}

impl<'a> FcnnAgent<'a> {
    pub(crate) fn new(hidden: usize, height: usize) -> FcnnAgent<'a> {
        let heights = |output: usize| {
            let mut vec = Vec::with_capacity(hidden + 2);
            vec.push(VECTOR_SIZE);
            vec.extend(std::iter::repeat(height).take(hidden));
            vec.push(output);
            vec
        };

        FcnnAgent {
            primary_network: FCNN::new_softmax(heights(7)),
            trade_network: FCNN::new_softmax(heights(4)),
            produce_network: FCNN::new_softmax(heights(5)),
            move_from_network: FCNN::new_softmax(heights(6)),
            move_to_network: FCNN::new_softmax(heights(5)),
            upgrade_from_network: FCNN::new_softmax(heights(7)),
            upgrade_to_network: FCNN::new_softmax(heights(4)),
            deploy_network: FCNN::new_softmax(heights(5)),
            build_network: FCNN::new_softmax(heights(5)),
            mill_network: FCNN::new_softmax(heights(5)),
            enlist_secondary_network: FCNN::new_softmax(heights(5)),
            enlist_onetime_network: FCNN::new_softmax(heights(4)),
        }
    }
}

impl Agent for FcnnAgent<'_> {
    fn prepare(&mut self, _state: &PlayerState) {}

    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction {
        let input = transform_state(state);
        let output = self.primary_network.predict(&input);
        let choice = output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        match choice {
            0 => PrimaryAction::Move,
            1 => PrimaryAction::Tax,
            2 => PrimaryAction::Trade,
            3 => PrimaryAction::Produce,
            4 => PrimaryAction::Promote,
            5 => PrimaryAction::Bolster,
            _ => PrimaryAction::Enforce,
        }
    }

    fn choose_trade(&self, state: &PlayerState) -> Resource {
        let input = transform_state(state);
        let output = self.trade_network.predict(&input);
        let choice = output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        match choice {
            0 => Wood,
            1 => Metal,
            2 => Oil,
            _ => Food,
        }
    }

    fn choose_produce(&self, state: &PlayerState) -> Resource {
        let input = transform_state(state);
        let output = self.produce_network.predict(&input);
        let choice = output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        match choice {
            0 => Wood,
            1 => Metal,
            2 => Oil,
            3 => Food,
            _ => People,
        }
    }

    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)> {
        let input = transform_state(state);
        let from_output = self.move_from_network.predict(&input);
        let from_choice = from_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        if from_choice == 5 {
            return None;
        }

        let from = match from_choice {
            0 => Wood,
            1 => Metal,
            2 => Oil,
            3 => Food,
            _ => People,
        };
        let to_output = self.move_to_network.predict(&input);
        let to_choice = to_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        let to = match to_choice {
            0 => Wood,
            1 => Metal,
            2 => Oil,
            3 => Food,
            _ => People,
        };

        Some((from, to))
    }

    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)> {
        let input = transform_state(state);
        let from_output = self.upgrade_from_network.predict(&input);
        let from_choice = from_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        if from_choice == 6 {
            return None;
        }

        let from = match from_choice {
            0 => Upgrade::Popularity,
            1 => Upgrade::Power,
            2 => Upgrade::Card,
            3 => Upgrade::Move,
            4 => Upgrade::Tax,
            _ => Upgrade::Produce,
        };
        let to_output = self.upgrade_to_network.predict(&input);
        let to_choice = to_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        let to = match to_choice {
            0 => SecondaryAction::Upgrade,
            1 => SecondaryAction::Deploy,
            2 => SecondaryAction::Build,
            _ => SecondaryAction::Enlist,
        };

        Some((from, to))
    }

    fn deploy(&self, state: &PlayerState) -> Option<Mech> {
        let input = transform_state(state);
        let output = self.deploy_network.predict(&input);
        let choice = output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        match choice {
            0 => Some(Mech::First),
            1 => Some(Mech::Second),
            2 => Some(Mech::Third),
            3 => Some(Mech::Fourth),
            _ => None,
        }
    }

    fn build(&self, state: &PlayerState) -> Option<Building> {
        let input = transform_state(state);
        let output = self.build_network.predict(&input);
        let choice = output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        match choice {
            0 => Some(Building::Mine),
            1 => Some(Building::Mill),
            2 => Some(Building::Armory),
            3 => Some(Building::Monument),
            _ => None,
        }
    }

    fn choose_mill_location(&self, state: &PlayerState) -> Resource {
        let input = transform_state(state);
        let output = self.mill_network.predict(&input);
        let choice = output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        match choice {
            0 => Wood,
            1 => Metal,
            2 => Oil,
            3 => Food,
            _ => People,
        }
    }

    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)> {
        let input = transform_state(state);
        let secondary_output = self.enlist_secondary_network.predict(&input);
        let secondary_choice = secondary_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        if secondary_choice == 4 {
            return None;
        }

        let secondary = match secondary_choice {
            0 => Recruit::Military,
            1 => Recruit::Coin,
            2 => Recruit::Popularity,
            _ => Recruit::Card,
        };

        let onetime_output = self.enlist_onetime_network.predict(&input);
        let onetime_choice = onetime_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        let onetime = match onetime_choice {
            0 => Recruit::Military,
            1 => Recruit::Coin,
            2 => Recruit::Popularity,
            _ => Recruit::Card,
        };

        Some((secondary, onetime))
    }
}

pub(crate) struct PredictiveQAgent<'a> {
    coin_network: FCNN<'a>,
}

impl PredictiveQAgent<'_> {
    pub(crate) fn new(hidden: usize, layout: Layout) -> Self {
        let layout = match layout {
            Layout::Fixed { height } => vec![height; hidden],
            Layout::Convolution {
                start,
                reduction,
                max_height,
            } => (0..hidden)
                .rev()
                .map(|i| max_height.min(start + i * reduction))
                .collect(),
        };

        let mut heights = Vec::with_capacity(hidden + 2);
        heights.push(VECTOR_SIZE);
        heights.extend(layout);
        heights.push(1);

        PredictiveQAgent {
            coin_network: FCNN::new(heights, &MLFunction::ELU),
        }
    }
}

pub(crate) enum Layout {
    Fixed {
        height: usize,
    },
    Convolution {
        start: usize,
        reduction: usize,
        max_height: usize,
    },
}

impl Agent for PredictiveQAgent<'_> {
    fn prepare(&mut self, _state: &PlayerState) {}

    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction {
        todo!()
    }

    fn choose_trade(&self, state: &PlayerState) -> Resource {
        todo!()
    }

    fn choose_produce(&self, state: &PlayerState) -> Resource {
        todo!()
    }

    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)> {
        todo!()
    }

    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)> {
        todo!()
    }

    fn deploy(&self, state: &PlayerState) -> Option<Mech> {
        todo!()
    }

    fn build(&self, state: &PlayerState) -> Option<Building> {
        todo!()
    }

    fn choose_mill_location(&self, state: &PlayerState) -> Resource {
        todo!()
    }

    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)> {
        todo!()
    }
}

const VECTOR_SIZE: usize = 64;

fn transform_state(state: &PlayerState) -> Array1<f64> {
    let mut result = Array1::zeros(VECTOR_SIZE);

    // basic stats
    result[0] = state.coins as f64;
    result[1] = state.resources.wood as f64;
    result[2] = state.resources.metal as f64;
    result[3] = state.resources.oil as f64;
    result[4] = state.resources.food as f64;
    result[5] = state.fields as f64;
    result[6] = state.cards as f64;
    result[7] = state.turns as f64;

    // upgrades
    result[8] = state.upgrades.popularity_evolved as u8 as f64;
    result[9] = state.upgrades.power_evolved as u8 as f64;
    result[10] = state.upgrades.card_evolved as u8 as f64;
    result[11] = state.upgrades.move_evolved as u8 as f64;
    result[12] = state.upgrades.tax_evolved as u8 as f64;
    result[13] = state.upgrades.produce_evolved as u8 as f64;
    result[14] = state.upgrades.star as u8 as f64;
    result[15] = state.upgrades.upgrade_base_cost as f64;
    result[16] = state.upgrades.upgrade_evolution_cost as f64;
    result[17] = state.upgrades.upgrade_coins as f64;
    result[18] = state.upgrades.deploy_base_cost as f64;
    result[19] = state.upgrades.deploy_evolution_cost as f64;
    result[20] = state.upgrades.deploy_coins as f64;
    result[21] = state.upgrades.build_base_cost as f64;
    result[22] = state.upgrades.build_evolution_cost as f64;
    result[23] = state.upgrades.build_coins as f64;
    result[24] = state.upgrades.enlist_base_cost as f64;
    result[25] = state.upgrades.enlist_evolution_cost as f64;
    result[26] = state.upgrades.enlist_coins as f64;

    // mechs
    result[27] = state.mechs.first_deployed as u8 as f64;
    result[28] = state.mechs.second_deployed as u8 as f64;
    result[29] = state.mechs.third_deployed as u8 as f64;
    result[30] = state.mechs.fourth_deployed as u8 as f64;
    result[31] = state.mechs.star as u8 as f64;

    // buildings
    result[32] = state.buildings.mine_built as u8 as f64;
    result[33] = state.buildings.mill_built as u8 as f64;
    result[34] = state.buildings.armory_built as u8 as f64;
    result[35] = state.buildings.monument_built as u8 as f64;
    result[36] = state.buildings.star as u8 as f64;
    result[37] = state.buildings.mill_location.is_none() as u8 as f64;
    result[38] = (state.buildings.mill_location == Some(Wood)) as u8 as f64;
    result[39] = (state.buildings.mill_location == Some(Metal)) as u8 as f64;
    result[40] = (state.buildings.mill_location == Some(Oil)) as u8 as f64;
    result[41] = (state.buildings.mill_location == Some(Food)) as u8 as f64;
    result[42] = (state.buildings.mill_location == Some(People)) as u8 as f64;

    // recruits
    result[43] = state.recruits.secondary_military_recruited as u8 as f64;
    result[44] = state.recruits.secondary_coin_recruited as u8 as f64;
    result[45] = state.recruits.secondary_popularity_recruited as u8 as f64;
    result[46] = state.recruits.secondary_card_recruited as u8 as f64;
    result[47] = state.recruits.onetime_military_recruited as u8 as f64;
    result[48] = state.recruits.onetime_coin_recruited as u8 as f64;
    result[49] = state.recruits.onetime_popularity_recruited as u8 as f64;
    result[50] = state.recruits.onetime_card_recruited as u8 as f64;
    result[51] = state.recruits.star as u8 as f64;

    // military
    result[52] = state.military.power as f64;
    result[53] = state.military.star as u8 as f64;

    // popularity
    result[54] = state.popularity.popularity as f64;
    result[55] = state.popularity.star as u8 as f64;
    result[56] = state.popularity.star_multiplier() as u8 as f64;
    result[57] = state.popularity.fields_multiplier() as u8 as f64;
    result[58] = state.popularity.resources_multiplier() as u8 as f64;

    // production
    result[59] = state.production.wood as f64;
    result[60] = state.production.metal as f64;
    result[61] = state.production.oil as f64;
    result[62] = state.production.food as f64;
    result[63] = state.production.population as f64;

    result
}
