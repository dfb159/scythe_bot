use core::panic;

use crate::agent::Agent;
use crate::game::turnhelper::turn;
use crate::game::turnmask::{Tile, TurnMask};
use crate::game::turnpredictor::get_actions;
use crate::game_state::PlayerState;
use crate::network::fcnn::{MLFunction, Predictor, Trainer, FCNN};

use ndarray::Array1;

pub struct PredictiveQAgent<'a> {
    coin_network: FCNN<'a>,
}

impl PredictiveQAgent<'_> {
    pub fn new(hidden: usize, layout: Layout) -> Self {
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

    pub fn predict(&self, state: &PlayerState) -> f64 {
        let vector = transform_state(state);
        let coin = self.coin_network.predict(&vector);
        coin[0]
    }

    pub fn train(&mut self, state: &PlayerState, gamma: f64, learning_rate: f64) {
        // Search the best prediction of the next move
        let (action, best_prediction) = self.max_turn(state);
        let new_state = turn(*state, &action);

        // Update the prediction by the Bellman equation with the best prediction
        let total_coins = new_state.total_coins() as f64;
        let new_prediction = total_coins + gamma * best_prediction;

        // Train the network with the new prediction. the leaning rate is replacing the alpha in the Bellman equation
        let input = transform_state(state);
        let target = Array1::from_elem(1, new_prediction);
        self.coin_network.train(&input, &target, learning_rate)
    }

    fn max_turn(&mut self, state: &PlayerState) -> (TurnMask, f64) {
        match get_actions(state)
            .iter()
            .map(|action| {
                let new_state = turn(state.clone(), action);
                let new_coin = self.predict(&new_state);
                (action, new_coin)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        {
            Some((mask, coins)) => (*mask, coins),
            None => panic!("No actions available!"),
        }
    }
}

pub enum Layout {
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
    fn get_action(&mut self, state: &PlayerState) -> TurnMask {
        let (action, _) = self.max_turn(state);
        action
    }
}

const VECTOR_SIZE: usize = 65;

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
    result[38] = (state.buildings.mill_location == Some(Tile::Woods)) as u8 as f64;
    result[39] = (state.buildings.mill_location == Some(Tile::Mountain)) as u8 as f64;
    result[40] = (state.buildings.mill_location == Some(Tile::Tundra)) as u8 as f64;
    result[41] = (state.buildings.mill_location == Some(Tile::Farm)) as u8 as f64;
    result[42] = (state.buildings.mill_location == Some(Tile::Village)) as u8 as f64;

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
    result[59] = state.production.get(&Tile::Woods) as f64;
    result[60] = state.production.get(&Tile::Mountain) as f64;
    result[61] = state.production.get(&Tile::Tundra) as f64;
    result[62] = state.production.get(&Tile::Farm) as f64;
    result[63] = state.production.get(&Tile::Village) as f64;

    result[64] = state.stars() as f64;

    result
}
