use rand::{seq::IteratorRandom, Rng};
use std::{cmp::min, io};

#[derive(Debug)]
struct PlayerState {
    player_name: String,
    faction_name: String,
    playstyle_name: String,

    move_secondary: SecondaryAction, // for move and tax primary actions
    trade_secondary: SecondaryAction, // for trade and promote primary actions
    produce_secondary: SecondaryAction, // for produce primary action
    bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    upgrades: UpgradesState,
    mechs: MechsState,
    buildings: BuildingsState,
    recruits: RecruitsState,
    military: MilitaryState,
    popularity: PopularityState,
    production: ProductionState,

    resources: ResourcesState,
    coins: i32,
    fields: i32,
    cards: i32, // TODO: change to actual BattleCards
    turns: i32,
}

impl PlayerState {
    fn new(player: &Player, faction: &Faction, player_mat: &PlayerMat) -> PlayerState {
        PlayerState {
            player_name: player.name.clone(),
            faction_name: faction.name.clone(),
            playstyle_name: player_mat.name.clone(),

            move_secondary: player_mat.move_secondary,
            trade_secondary: player_mat.trade_secondary,
            produce_secondary: player_mat.produce_secondary,
            bolster_secondary: player_mat.bolster_secondary,

            upgrades: UpgradesState::new(&player_mat),
            mechs: MechsState::new(),
            buildings: BuildingsState::new(),
            recruits: RecruitsState::new(),
            military: MilitaryState::new(faction.starting_power + player.bonus_starting_power),
            popularity: PopularityState::new(
                player_mat.starting_popularity + player.bonus_starting_popularity,
            ),
            production: ProductionState::new(
                faction.first_starting_field,
                faction.second_starting_field,
            ),

            resources: ResourcesState::new(),
            coins: player_mat.starting_coins + player.bonus_starting_coins,
            fields: 2,
            cards: faction.starting_cards,
            turns: 0,
        }
    }

    fn stars(&self) -> i32 {
        [
            self.upgrades.star,
            self.mechs.star,
            self.buildings.star,
            self.recruits.star,
            self.military.star,
            self.popularity.star,
        ]
        .iter()
        .fold(0, |acc, &star| acc + if star { 1 } else { 0 })
    }

    fn has_won(&self) -> bool {
        self.stars() >= 6
    }

    fn total_coins(&self) -> i32 {
        self.coins
            + self.stars() * self.popularity.star_multiplier()
            + self.fields * self.popularity.fields_multiplier()
            + self.resources.total() / 2 * self.popularity.resources_multiplier()
    }

    fn get_secondary(&self, primary: PrimaryAction) -> SecondaryAction {
        match primary {
            PrimaryAction::Move => self.move_secondary,
            PrimaryAction::Tax => self.move_secondary,
            PrimaryAction::Trade => self.trade_secondary,
            PrimaryAction::Promote => self.trade_secondary,
            PrimaryAction::Bolster => self.bolster_secondary,
            PrimaryAction::Enforce => self.bolster_secondary,
            PrimaryAction::Produce => self.produce_secondary,
        }
    }

    fn can_produce(&self) -> bool {
        let total = self.production.total();
        if total >= 8 && self.coins <= 0 {
            return false;
        }
        if total >= 6 && self.popularity.popularity <= 0 {
            return false;
        }
        if total >= 4 && self.military.power <= 0 {
            return false;
        }
        true
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    bonus_starting_coins: i32,
    bonus_starting_power: i32,
    bonus_starting_popularity: i32,
}

#[derive(Debug)]
struct Faction {
    name: String,
    starting_power: i32,
    starting_cards: i32,

    first_starting_field: Resource,
    second_starting_field: Resource,
}

#[derive(Debug)]
struct PlayerMat {
    name: String,
    starting_index: i32,

    starting_coins: i32,
    starting_popularity: i32,

    move_secondary: SecondaryAction, // for move and tax primary actions
    trade_secondary: SecondaryAction, // for trade and promote primary actions
    produce_secondary: SecondaryAction, // for produce primary action
    bolster_secondary: SecondaryAction, // for bolster and enforce primary actions

    upgrade_cost: i32,
    upgrade_evolutions: i32,
    upgrade_coins: i32,
    deploy_cost: i32,
    deploy_evolutions: i32,
    deploy_coins: i32,
    build_cost: i32,
    build_evolutions: i32,
    build_coins: i32,
    enlist_cost: i32,
    enlist_evolutions: i32,
    enlist_coins: i32,
}

#[derive(Debug)]
struct ProductionState {
    wood: i32,
    metal: i32,
    oil: i32,
    food: i32,
    population: i32,
}

impl ProductionState {
    fn new(first_field: Resource, second_field: Resource) -> ProductionState {
        let mut state = ProductionState {
            wood: 0,
            metal: 0,
            oil: 0,
            food: 0,
            population: 0,
        };

        state.add(first_field, 1);
        state.add(second_field, 1);

        state
    }

    fn add(&mut self, resource: Resource, amount: i32) {
        let reduced = min(amount, 8 - self.total());

        match resource {
            Resource::Wood => {
                self.wood += reduced;
            }
            Resource::Metal => {
                self.metal += reduced;
            }
            Resource::Oil => {
                self.oil += reduced;
            }
            Resource::Food => {
                self.food += reduced;
            }
            Resource::People => {
                self.population += reduced;
            }
        }
    }

    fn get(&self, resource: Resource) -> i32 {
        match resource {
            Resource::Wood => self.wood,
            Resource::Metal => self.metal,
            Resource::Oil => self.oil,
            Resource::Food => self.food,
            Resource::People => self.population,
        }
    }

    fn total(&self) -> i32 {
        self.wood + self.metal + self.oil + self.food + self.population
    }
}

#[derive(Debug)]
struct PopularityState {
    popularity: i32,
    star: bool,
}

impl PopularityState {
    fn new(popularity: i32) -> PopularityState {
        PopularityState {
            popularity,
            star: false,
        }
    }

    fn add(&mut self, popularity: i32) {
        self.set(self.popularity + popularity);
    }

    fn set(&mut self, popularity: i32) {
        self.popularity = popularity;
        if self.popularity >= 18 {
            self.popularity = 18;
            self.star = true;
        }
        if self.popularity < 0 {
            self.popularity = 0;
        }
    }

    fn star_multiplier(&self) -> i32 {
        if self.popularity < 7 {
            3
        } else if self.popularity < 14 {
            4
        } else {
            5
        }
    }

    fn fields_multiplier(&self) -> i32 {
        if self.popularity < 7 {
            2
        } else if self.popularity < 14 {
            3
        } else {
            4
        }
    }

    fn resources_multiplier(&self) -> i32 {
        if self.popularity < 7 {
            1
        } else if self.popularity < 14 {
            2
        } else {
            3
        }
    }
}

#[derive(Debug)]
struct ResourcesState {
    wood: i32,
    metal: i32,
    oil: i32,
    food: i32,
}

impl ResourcesState {
    fn new() -> ResourcesState {
        ResourcesState {
            wood: 0,
            metal: 0,
            oil: 0,
            food: 0,
        }
    }

    fn total(&self) -> i32 {
        self.wood + self.metal + self.oil + self.food
    }

    fn add(&mut self, resource: Resource, amount: i32) {
        match resource {
            Resource::Wood => {
                self.wood += amount;
            }
            Resource::Metal => {
                self.metal += amount;
            }
            Resource::Oil => {
                self.oil += amount;
            }
            Resource::Food => {
                self.food += amount;
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Resource {
    Wood,
    Metal,
    Oil,
    Food,
    People,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Upgrade {
    Popularity,
    Power,
    Card,
    Move,
    Tax,
    Produce,
}

#[derive(Debug)]
struct UpgradesState {
    popularity_evolved: bool,
    power_evolved: bool,
    card_evolved: bool,
    move_evolved: bool,
    tax_evolved: bool,
    produce_evolved: bool,

    upgrade_base_cost: i32,
    upgrade_evolution_cost: i32,
    upgrade_coins: i32,
    deploy_base_cost: i32,
    deploy_evolution_cost: i32,
    deploy_coins: i32,
    build_base_cost: i32,
    build_evolution_cost: i32,
    build_coins: i32,
    enlist_base_cost: i32,
    enlist_evolution_cost: i32,
    enlist_coins: i32,

    star: bool,
}

impl UpgradesState {
    fn new(mat: &PlayerMat) -> UpgradesState {
        UpgradesState {
            popularity_evolved: false,
            power_evolved: false,
            card_evolved: false,
            move_evolved: false,
            tax_evolved: false,
            produce_evolved: false,

            upgrade_base_cost: mat.upgrade_cost,
            upgrade_evolution_cost: mat.upgrade_evolutions,
            upgrade_coins: mat.upgrade_coins,
            deploy_base_cost: mat.deploy_cost,
            deploy_evolution_cost: mat.deploy_evolutions,
            deploy_coins: mat.deploy_coins,
            build_base_cost: mat.build_cost,
            build_evolution_cost: mat.build_evolutions,
            build_coins: mat.build_coins,
            enlist_base_cost: mat.enlist_cost,
            enlist_evolution_cost: mat.enlist_evolutions,
            enlist_coins: mat.enlist_coins,

            star: false,
        }
    }

    fn upgrade(&mut self, primary: Upgrade, secondary: SecondaryAction) {
        match primary {
            Upgrade::Popularity => {
                self.popularity_evolved = true;
            }
            Upgrade::Power => {
                self.power_evolved = true;
            }
            Upgrade::Card => {
                self.card_evolved = true;
            }
            Upgrade::Move => {
                self.move_evolved = true;
            }
            Upgrade::Tax => {
                self.tax_evolved = true;
            }
            Upgrade::Produce => {
                self.produce_evolved = true;
            }
        }
        match secondary {
            SecondaryAction::Upgrade => {
                if self.upgrade_evolution_cost > 0 {
                    self.upgrade_evolution_cost -= 1;
                }
            }
            SecondaryAction::Deploy => {
                if self.deploy_evolution_cost > 0 {
                    self.deploy_evolution_cost -= 1;
                }
            }
            SecondaryAction::Build => {
                if self.build_evolution_cost > 0 {
                    self.build_evolution_cost -= 1;
                }
            }
            SecondaryAction::Enlist => {
                if self.enlist_evolution_cost > 0 {
                    self.enlist_evolution_cost -= 1;
                }
            }
        }
        if self.popularity_evolved
            && self.power_evolved
            && self.card_evolved
            && self.move_evolved
            && self.tax_evolved
            && self.produce_evolved
        {
            self.star = true;
        }
    }

    fn can_upgrade(&self, primary: Upgrade, secondary: SecondaryAction) -> bool {
        self.can_upgrade_primary(primary) && self.can_upgrade_secondary(secondary)
    }

    fn can_upgrade_primary(&self, primary: Upgrade) -> bool {
        match primary {
            Upgrade::Popularity => !self.popularity_evolved,
            Upgrade::Power => !self.power_evolved,
            Upgrade::Card => !self.card_evolved,
            Upgrade::Move => !self.move_evolved,
            Upgrade::Tax => !self.tax_evolved,
            Upgrade::Produce => !self.produce_evolved,
        }
    }

    fn can_upgrade_secondary(&self, secondary: SecondaryAction) -> bool {
        match secondary {
            SecondaryAction::Upgrade => self.upgrade_evolution_cost > 0,
            SecondaryAction::Deploy => self.deploy_evolution_cost > 0,
            SecondaryAction::Build => self.build_evolution_cost > 0,
            SecondaryAction::Enlist => self.enlist_evolution_cost > 0,
        }
    }

    fn get_upgrade_cost(&self, secondary: SecondaryAction) -> i32 {
        match secondary {
            SecondaryAction::Upgrade => self.upgrade_base_cost + self.upgrade_evolution_cost,
            SecondaryAction::Deploy => self.deploy_base_cost + self.deploy_evolution_cost,
            SecondaryAction::Build => self.build_base_cost + self.build_evolution_cost,
            SecondaryAction::Enlist => self.enlist_base_cost + self.enlist_evolution_cost,
        }
    }

    fn get_upgrade_coins(&self, secondary: SecondaryAction) -> i32 {
        match secondary {
            SecondaryAction::Upgrade => self.upgrade_coins,
            SecondaryAction::Deploy => self.deploy_coins,
            SecondaryAction::Build => self.build_coins,
            SecondaryAction::Enlist => self.enlist_coins,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Mech {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug)]
struct MechsState {
    first_deployed: bool,
    second_deployed: bool,
    third_deployed: bool,
    fourth_deployed: bool,
    star: bool,
}

impl MechsState {
    fn new() -> MechsState {
        MechsState {
            first_deployed: false,
            second_deployed: false,
            third_deployed: false,
            fourth_deployed: false,
            star: false,
        }
    }

    fn deploy(&mut self, mech: Mech) {
        match mech {
            Mech::First => {
                self.first_deployed = true;
            }
            Mech::Second => {
                self.second_deployed = true;
            }
            Mech::Third => {
                self.third_deployed = true;
            }
            Mech::Fourth => {
                self.fourth_deployed = true;
            }
        }
        if self.first_deployed
            && self.second_deployed
            && self.third_deployed
            && self.fourth_deployed
        {
            self.star = true;
        }
    }

    fn can_deploy(&self, mech: Mech) -> bool {
        match mech {
            Mech::First => !self.first_deployed,
            Mech::Second => !self.second_deployed,
            Mech::Third => !self.third_deployed,
            Mech::Fourth => !self.fourth_deployed,
        }
    }
}

#[derive(Debug)]
struct MilitaryState {
    power: i32,
    star: bool,
}

impl MilitaryState {
    fn new(power: i32) -> MilitaryState {
        MilitaryState { power, star: false }
    }

    fn add(&mut self, power: i32) {
        self.set(self.power + power);
    }

    fn set(&mut self, power: i32) {
        self.power = power;
        if self.power >= 16 {
            self.power = 16;
            self.star = true;
        }
        if self.power < 0 {
            self.power = 0;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Building {
    Mine,
    Mill,
    Armory,
    Monument,
}

#[derive(Debug)]
struct BuildingsState {
    mine_build: bool,
    mill_build: bool,
    armory_build: bool,
    monument_build: bool,

    mill_location: Option<Resource>,

    star: bool,
}

impl BuildingsState {
    fn new() -> BuildingsState {
        BuildingsState {
            mine_build: false,
            mill_build: false,
            armory_build: false,
            monument_build: false,

            mill_location: None,

            star: false,
        }
    }

    fn build(&mut self, building: Building) {
        match building {
            Building::Mine => {
                self.mine_build = true;
            }
            Building::Mill => {
                self.mill_build = true;
            }
            Building::Armory => {
                self.armory_build = true;
            }
            Building::Monument => {
                self.monument_build = true;
            }
        }
        if self.mine_build && self.mill_build && self.armory_build && self.monument_build {
            self.star = true;
        }
    }

    fn can_build(&self, building: Building) -> bool {
        match building {
            Building::Mine => !self.mine_build,
            Building::Mill => !self.mill_build,
            Building::Armory => !self.armory_build,
            Building::Monument => !self.monument_build,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Recruit {
    Military,
    Coin,
    Popularity,
    Card,
}

#[derive(Debug)]
struct RecruitsState {
    secondary_military_recruited: bool,
    secondary_coin_recruited: bool,
    secondary_popularity_recruited: bool,
    secondary_card_recruited: bool,

    onetime_military_recruited: bool,
    onetime_coin_recruited: bool,
    onetime_popularity_recruited: bool,
    onetime_card_recruited: bool,

    star: bool,
}

impl RecruitsState {
    fn new() -> RecruitsState {
        RecruitsState {
            secondary_military_recruited: false,
            secondary_coin_recruited: false,
            secondary_popularity_recruited: false,
            secondary_card_recruited: false,

            onetime_military_recruited: false,
            onetime_coin_recruited: false,
            onetime_popularity_recruited: false,
            onetime_card_recruited: false,

            star: false,
        }
    }

    fn recruit(&mut self, secondary: Recruit, onetime: Recruit) {
        match secondary {
            Recruit::Military => {
                self.secondary_military_recruited = true;
            }
            Recruit::Coin => {
                self.secondary_coin_recruited = true;
            }
            Recruit::Popularity => {
                self.secondary_popularity_recruited = true;
            }
            Recruit::Card => {
                self.secondary_card_recruited = true;
            }
        }
        match onetime {
            Recruit::Military => {
                self.onetime_military_recruited = true;
            }
            Recruit::Coin => {
                self.onetime_coin_recruited = true;
            }
            Recruit::Popularity => {
                self.onetime_popularity_recruited = true;
            }
            Recruit::Card => {
                self.onetime_card_recruited = true;
            }
        }
        if self.secondary_military_recruited
            && self.secondary_coin_recruited
            && self.secondary_popularity_recruited
            && self.secondary_card_recruited
            && self.onetime_military_recruited
            && self.onetime_coin_recruited
            && self.onetime_popularity_recruited
            && self.onetime_card_recruited
        {
            self.star = true;
        }
    }

    fn can_recruit(&self, secondary: Recruit, onetime: Recruit) -> bool {
        !self.is_secondary_recruited(secondary) && !self.is_onetime_recruited(onetime)
    }

    fn is_secondary_recruited(&self, secondary: Recruit) -> bool {
        match secondary {
            Recruit::Military => self.secondary_military_recruited,
            Recruit::Coin => self.secondary_coin_recruited,
            Recruit::Popularity => self.secondary_popularity_recruited,
            Recruit::Card => self.secondary_card_recruited,
        }
    }

    fn is_onetime_recruited(&self, onetime: Recruit) -> bool {
        match onetime {
            Recruit::Military => self.onetime_military_recruited,
            Recruit::Coin => self.onetime_coin_recruited,
            Recruit::Popularity => self.onetime_popularity_recruited,
            Recruit::Card => self.onetime_card_recruited,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PrimaryAction {
    Move,
    Tax,
    Trade,
    Promote,
    Bolster,
    Enforce,
    Produce,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum SecondaryAction {
    Upgrade,
    Deploy,

    Build,
    Enlist,
}

trait Agent {
    fn prepare(&self, state: &PlayerState);
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

struct RandomAgent {}

impl Agent for RandomAgent {
    fn prepare(&self, _state: &PlayerState) {}

    fn choose_primary(&self, state: &PlayerState) -> PrimaryAction {
        let mut choice = Vec::with_capacity(7);
        if state.can_produce() {
            choice.push(PrimaryAction::Produce);
        }
        if state.coins >= 1 {
            choice.push(PrimaryAction::Trade);
            choice.push(PrimaryAction::Promote);
            choice.push(PrimaryAction::Bolster);
            choice.push(PrimaryAction::Enforce);
        }
        choice.push(PrimaryAction::Move);
        choice.push(PrimaryAction::Tax);

        choice.into_iter().choose(&mut rand::thread_rng()).unwrap()
    }

    fn choose_trade(&self, _state: &PlayerState) -> Resource {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            _ => Resource::Food,
        }
    }

    fn choose_produce(&self, _state: &PlayerState) -> Resource {
        match rand::thread_rng().gen_range(0..=4) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            3 => Resource::Food,
            _ => Resource::People,
        }
    }

    fn choose_move(&self, state: &PlayerState) -> Option<(Resource, Resource)> {
        let from = vec![
            Resource::Wood,
            Resource::Metal,
            Resource::Oil,
            Resource::Food,
            Resource::People,
        ]
        .into_iter()
        .filter(|resource| state.production.get(*resource) > 0)
        .choose(&mut rand::thread_rng());

        let to = match rand::thread_rng().gen_range(0..=4) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            3 => Resource::Food,
            _ => Resource::People,
        };

        match from {
            Some(from) if from != to => Some((from, to)),
            _ => None,
        }
    }

    fn upgrade(&self, state: &PlayerState) -> Option<(Upgrade, SecondaryAction)> {
        let primary_list = vec![
            Upgrade::Popularity,
            Upgrade::Power,
            Upgrade::Card,
            Upgrade::Move,
            Upgrade::Tax,
            Upgrade::Produce,
        ]
        .into_iter()
        .filter(|upgrade| state.upgrades.can_upgrade_primary(*upgrade))
        .choose(&mut rand::thread_rng());

        let secondary_list = vec![
            SecondaryAction::Upgrade,
            SecondaryAction::Deploy,
            SecondaryAction::Build,
            SecondaryAction::Enlist,
        ]
        .into_iter()
        .filter(|secondary| state.upgrades.can_upgrade_secondary(*secondary))
        .choose(&mut rand::thread_rng());

        match (primary_list, secondary_list) {
            (Some(primary), Some(secondary)) => Some((primary, secondary)),
            _ => None,
        }
    }

    fn deploy(&self, state: &PlayerState) -> Option<Mech> {
        let mech_list = vec![Mech::First, Mech::Second, Mech::Third, Mech::Fourth]
            .into_iter()
            .filter(|mech| state.mechs.can_deploy(*mech))
            .choose(&mut rand::thread_rng());

        match mech_list {
            Some(mech) => Some(mech),
            _ => None,
        }
    }

    fn build(&self, state: &PlayerState) -> Option<Building> {
        let building_list = vec![
            Building::Mine,
            Building::Mill,
            Building::Armory,
            Building::Monument,
        ]
        .into_iter()
        .filter(|building| state.buildings.can_build(*building))
        .choose(&mut rand::thread_rng());

        match building_list {
            Some(building) => Some(building),
            _ => None,
        }
    }

    fn choose_mill_location(&self, _state: &PlayerState) -> Resource {
        match rand::thread_rng().gen_range(0..=4) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            3 => Resource::Food,
            _ => Resource::People,
        }
    }

    fn enlist(&self, state: &PlayerState) -> Option<(Recruit, Recruit)> {
        let secondary_list = vec![
            Recruit::Military,
            Recruit::Coin,
            Recruit::Popularity,
            Recruit::Card,
        ]
        .into_iter()
        .filter(|recruit| !state.recruits.is_secondary_recruited(*recruit))
        .choose(&mut rand::thread_rng());

        let onetime_list = vec![
            Recruit::Military,
            Recruit::Coin,
            Recruit::Popularity,
            Recruit::Card,
        ]
        .into_iter()
        .filter(|recruit| !state.recruits.is_onetime_recruited(*recruit))
        .choose(&mut rand::thread_rng());

        match (secondary_list, onetime_list) {
            (Some(secondary), Some(onetime)) => Some((secondary, onetime)),
            _ => None,
        }
    }
}

fn main() {
    println!("Welcome to scythe statistics!");

    let joey = Player {
        name: "Joey".to_string(),
        bonus_starting_coins: 0,
        bonus_starting_power: 0,
        bonus_starting_popularity: 0,
    };

    let rusviet = Faction {
        name: "Rusviet".to_string(),
        starting_power: 3,
        starting_cards: 1,
        first_starting_field: Resource::Wood,
        second_starting_field: Resource::Metal,
    };

    let player_mat = PlayerMat {
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

    let mut state = PlayerState::new(&joey, &rusviet, &player_mat);
    let agent = RandomAgent {};

    while state.turns <= 500 && !state.has_won() {
        turn(&mut state, &agent);
    }

    println!(
        "Congratulations! You have won the game in {} turns! {} has scored {} coins!",
        state.turns,
        state.faction_name,
        state.total_coins()
    );
    println!("{state:?}");
}

fn turn(state: &mut PlayerState, agent: &impl Agent) {
    agent.prepare(state);

    let choice = agent.choose_primary(state);
    execute_primary(state, agent, choice);

    state.turns += 1;

    let secondary = state.get_secondary(choice);
    execute_secondary(state, agent, secondary);

    println!("Turn: {} == Action: {:?}", state.turns, choice);
    //println!("{state:#?}");
    //println!("Turn: {} == Action: {:?}", state.turns, choice);
    //println!();
    //
    //let mut input = String::new();
    //io::stdin()
    //    .read_line(&mut input)
    //    .expect("Failed to read the line!");
}

fn execute_primary(state: &mut PlayerState, agent: &impl Agent, primary: PrimaryAction) {
    match primary {
        PrimaryAction::Move => {
            move_people(state, agent.choose_move(state));
            move_people(state, agent.choose_move(state));
            if state.upgrades.move_evolved {
                move_people(state, agent.choose_move(state));
            }
        }
        PrimaryAction::Tax => {
            state.coins += if state.upgrades.tax_evolved { 2 } else { 1 };
        }
        PrimaryAction::Trade if state.coins >= 1 => {
            state.coins -= 1;
            state.resources.add(agent.choose_trade(state), 1);
            state.resources.add(agent.choose_trade(state), 1);
            if state.buildings.armory_build {
                state.military.add(1);
            }
        }
        PrimaryAction::Promote if state.coins >= 1 => {
            state.coins -= 1;
            let popularity_increase = if state.upgrades.popularity_evolved {
                2
            } else {
                1
            };
            state.popularity.add(popularity_increase);

            if state.buildings.armory_build {
                state.military.add(1);
            }
        }
        PrimaryAction::Bolster if state.coins >= 1 => {
            state.coins -= 1;
            let power_increase = if state.upgrades.power_evolved { 3 } else { 2 };
            state.military.add(power_increase);

            if state.buildings.monument_build {
                state.popularity.add(1);
            }
        }
        PrimaryAction::Enforce if state.coins >= 1 => {
            state.coins -= 1;
            let card_increase = if state.upgrades.card_evolved { 2 } else { 1 };
            state.cards += card_increase;

            if state.buildings.monument_build {
                state.popularity.add(1);
            }
        }
        PrimaryAction::Produce if state.can_produce() => {
            // move check GameState Helper class
            let total = state.production.total();
            if total >= 4 {
                state.military.add(-1)
            }
            if total >= 6 {
                state.popularity.add(-1)
            }
            if total >= 8 {
                state.coins -= 1
            }

            produce_resource(state, agent.choose_produce(state));
            produce_resource(state, agent.choose_produce(state));

            if state.upgrades.produce_evolved {
                produce_resource(state, agent.choose_produce(state));
            }

            match state.buildings.mill_location {
                Some(location) => {
                    produce_resource(state, location);
                }
                None => {}
            }
        }
        _ => {
            panic!("Action was invalid, not enough ressources for it!")
        } // action invalid
    }
}

fn move_people(state: &mut PlayerState, dest: Option<(Resource, Resource)>) {
    match dest {
        Some((from, to)) if state.production.get(from) > 0 => {
            state.production.add(from, -1);
            state.production.add(to, 1);
        }
        _ => {}
    }
}

fn produce_resource(state: &mut PlayerState, resource: Resource) {
    match resource {
        Resource::People => {
            state
                .production
                .add(Resource::People, state.production.population);
        }
        _ => {
            state
                .resources
                .add(resource, state.production.get(resource));
        }
    }
}

fn execute_secondary(state: &mut PlayerState, agent: &impl Agent, secondary: SecondaryAction) {
    let cost = state.upgrades.get_upgrade_cost(secondary);
    match secondary {
        SecondaryAction::Upgrade if state.resources.oil >= cost => match agent.upgrade(state) {
            Some((primary, secondary)) => {
                if state.recruits.is_secondary_recruited(Recruit::Military) {
                    state.military.add(1);
                }
                state.resources.oil -= cost;
                state.upgrades.upgrade(primary, secondary);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Upgrade);
            }
            _ => {}
        },
        SecondaryAction::Deploy if state.resources.metal >= cost => match agent.deploy(state) {
            Some(mech) => {
                if state.recruits.is_secondary_recruited(Recruit::Coin) {
                    state.coins += 1;
                }
                state.resources.metal -= cost;
                state.mechs.deploy(mech);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Deploy);
            }
            _ => {}
        },
        SecondaryAction::Build if state.resources.wood >= cost => match agent.build(state) {
            Some(Building::Mill) => {
                if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                    state.popularity.add(1);
                }
                state.resources.wood -= cost;
                state.buildings.build(Building::Mill);
                state.buildings.mill_location = Some(agent.choose_mill_location(state));
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Build);
            }
            Some(building) => {
                if state.recruits.is_secondary_recruited(Recruit::Popularity) {
                    state.popularity.add(1);
                }
                state.resources.wood -= cost;
                state.buildings.build(building);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Build);
            }
            _ => {}
        },
        SecondaryAction::Enlist if state.resources.food >= cost => match agent.enlist(state) {
            Some((secondary, onetime)) => {
                if state.recruits.is_secondary_recruited(Recruit::Card) {
                    state.cards += 1;
                }
                state.resources.food -= cost;
                state.recruits.recruit(secondary, onetime);
                state.coins += state.upgrades.get_upgrade_coins(SecondaryAction::Enlist);
            }
            _ => {}
        },
        _ => {}
    }
}
