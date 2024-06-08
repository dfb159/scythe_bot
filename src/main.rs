use std::io;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
struct Nation {
    name: String,
    player: String,
    evolutions: Evolutions,
    mechs: Mechs,
    buildings: Buildings,
    recruits: Recruits,
    coins: i32,
    military: Military,
    popularity: Popularity,
    resources: Resources,
    fields: i32,
    turns: i32,
}

impl Nation {
    fn new(name: &str, player: &str) -> Nation {
        Nation {
            name: String::from(name),
            player: String::from(player),
            evolutions: Evolutions::new(),
            mechs: Mechs::new(),
            buildings: Buildings::new(),
            recruits: Recruits::new(),
            coins: 5,
            military: Military::new(2),
            popularity: Popularity::new(3),
            resources: Resources::new(),
            fields: 2,
            turns: 0,
        }
    }

    fn stars(&self) -> i32 {
        let mut stars = 0;
        if self.evolutions.star {
            stars += 1;
        }
        if self.mechs.star {
            stars += 1;
        }
        if self.buildings.star {
            stars += 1;
        }
        if self.recruits.star {
            stars += 1;
        }
        if self.military.star {
            stars += 1;
        }
        if self.popularity.star {
            stars += 1;
        }
        stars
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
}

#[derive(Debug)]
struct Popularity {
    popularity: i32,
    star: bool,
}

impl Popularity {
    fn new(popularity: i32) -> Popularity {
        Popularity {
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
struct Resources {
    wood: i32,
    metal: i32,
    oil: i32,
    food: i32,
}

impl Resources {
    fn new() -> Resources {
        Resources {
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
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Resource {
    Wood,
    Metal,
    Oil,
    Food,
}

impl Distribution<Resource> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Resource {
        match rng.gen_range(0..=3) {
            0 => Resource::Wood,
            1 => Resource::Metal,
            2 => Resource::Oil,
            _ => Resource::Food,
        }
    }
}

#[derive(Debug)]
struct Evolutions {
    evolutions: i32,
    star: bool,
}

impl Evolutions {
    fn new() -> Evolutions {
        Evolutions {
            evolutions: 0,
            star: false,
        }
    }

    fn add(&mut self, evolutions: i32) {
        self.set(self.evolutions + evolutions);
    }

    fn is_max(&self) -> bool {
        self.evolutions >= 6
    }

    fn set(&mut self, evolutions: i32) {
        self.evolutions = evolutions;
        if self.evolutions >= 6 {
            self.evolutions = 6;
            self.star = true;
        }
        if self.evolutions < 0 {
            self.evolutions = 0;
        }
    }
}

#[derive(Debug)]
struct Mechs {
    mechs: i32,
    star: bool,
}

impl Mechs {
    fn new() -> Mechs {
        Mechs {
            mechs: 0,
            star: false,
        }
    }

    fn add(&mut self, mechs: i32) {
        self.set(self.mechs + mechs);
    }

    fn is_max(&self) -> bool {
        self.mechs >= 4
    }

    fn set(&mut self, mechs: i32) {
        self.mechs = mechs;
        if self.mechs >= 4 {
            self.mechs = 4;
            self.star = true;
        }
        if self.mechs < 0 {
            self.mechs = 0;
        }
    }
}

#[derive(Debug)]
struct Military {
    power: i32,
    star: bool,
}

impl Military {
    fn new(power: i32) -> Military {
        Military { power, star: false }
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

#[derive(Debug)]
struct Buildings {
    buildings: i32,
    star: bool,
}

impl Buildings {
    fn new() -> Buildings {
        Buildings {
            buildings: 0,
            star: false,
        }
    }

    fn add(&mut self, buildings: i32) {
        self.set(self.buildings + buildings);
    }

    fn is_max(&self) -> bool {
        self.buildings >= 4
    }

    fn set(&mut self, buildings: i32) {
        self.buildings = buildings;
        if self.buildings >= 4 {
            self.buildings = 4;
            self.star = true;
        }
        if self.buildings < 0 {
            self.buildings = 0;
        }
    }
}

#[derive(Debug)]
struct Recruits {
    recruits: i32,
    star: bool,
}

impl Recruits {
    fn new() -> Recruits {
        Recruits {
            recruits: 0,
            star: false,
        }
    }

    fn is_max(&self) -> bool {
        self.recruits >= 4
    }

    fn add(&mut self, recruits: i32) {
        self.set(self.recruits + recruits);
    }

    fn set(&mut self, recruits: i32) {
        self.recruits = recruits;
        if self.recruits >= 4 {
            self.recruits = 4;
            self.star = true;
        }
        if self.recruits < 0 {
            self.recruits = 0;
        }
    }
}

#[derive(Debug)]
enum PrimaryAction {
    Move,
    Tax,
    Produce,
    Trade,
    Promote,
    Bolster,
}

trait Agent {
    fn prepare(&self, nation: &Nation);
    fn choose_primary(&self) -> PrimaryAction;
    fn do_evolution(&self) -> bool;
    fn do_mech(&self) -> bool;
    fn do_building(&self) -> bool;
    fn do_recruit(&self) -> bool;
    fn choose_trade(&self) -> Resource;
    fn choose_produce(&self) -> Resource;
}

struct RandomAgent {}

impl Agent for RandomAgent {
    fn prepare(&self, _nation: &Nation) {}

    fn choose_primary(&self) -> PrimaryAction {
        match rand::thread_rng().gen_range(0..=5) {
            0 => PrimaryAction::Move,
            1 => PrimaryAction::Tax,
            2 => PrimaryAction::Produce,
            3 => PrimaryAction::Trade,
            4 => PrimaryAction::Promote,
            _ => PrimaryAction::Bolster,
        }
    }

    fn do_evolution(&self) -> bool {
        true
    }

    fn do_mech(&self) -> bool {
        true
    }

    fn do_building(&self) -> bool {
        true
    }

    fn do_recruit(&self) -> bool {
        true
    }

    fn choose_trade(&self) -> Resource {
        rand::random::<Resource>()
    }

    fn choose_produce(&self) -> Resource {
        rand::random::<Resource>()
    }
}

fn main() {
    println!("Welcome to scythe statistics!");

    let mut nation = Nation::new("Rusviet", "Joey");
    let agent = RandomAgent {};

    while !nation.has_won() {
        agent.prepare(&nation);

        let choice = agent.choose_primary();
        /*
        println!("Turn: {} == Action: {:?}", nation.turns, choice);
        println!("{nation:?}");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line!");
        */
        match choice {
            PrimaryAction::Move => {
                nation.fields += 1;

                nation.turns += 1;

                if nation.resources.oil >= 3 && agent.do_evolution() {
                    nation.resources.oil -= 3;
                    nation.evolutions.add(1);
                }
            }
            PrimaryAction::Tax => {
                nation.coins += 3;

                nation.turns += 1;

                if nation.resources.oil >= 3 && agent.do_evolution() {
                    nation.resources.oil -= 3;
                    nation.evolutions.add(1);
                }
            }
            PrimaryAction::Produce => {
                nation.resources.add(agent.choose_produce(), 2);

                nation.turns += 1;

                if nation.resources.metal >= 3 && agent.do_mech() {
                    nation.resources.metal -= 3;
                    nation.mechs.add(1);
                }
            }
            PrimaryAction::Trade => {
                if nation.coins <= 0 {
                    continue;
                }

                nation.coins -= 1;
                nation.resources.add(agent.choose_trade(), 1);
                nation.resources.add(agent.choose_trade(), 1);

                nation.turns += 1;

                if nation.resources.wood >= 3 && agent.do_building() {
                    nation.resources.wood -= 3;
                    nation.buildings.add(1);
                }
            }
            PrimaryAction::Promote => {
                if nation.coins <= 0 {
                    continue;
                }

                nation.coins -= 1;
                nation.popularity.add(1);

                nation.turns += 1;

                if nation.resources.wood >= 3 && agent.do_building() {
                    nation.resources.wood -= 3;
                    nation.buildings.add(1);
                }
            }
            PrimaryAction::Bolster => {
                nation.military.set(nation.military.power + 2);

                nation.turns += 1;

                if nation.resources.food >= 3 && agent.do_recruit() {
                    nation.resources.food -= 3;
                    nation.recruits.add(1);
                }
            }
        }
    }

    println!(
        "Congratulations! You have won the game in {} turns! {} has scored {} coins!",
        nation.turns,
        nation.name,
        nation.total_coins()
    );
    println!("{nation:?}");
}
