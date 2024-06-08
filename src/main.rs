use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{collections::HashMap, hash::Hash, io};

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
enum Action {
    Move,
    Produce,
    Trade,
    Empower,
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0..=3) {
            0 => Action::Move,
            1 => Action::Produce,
            2 => Action::Trade,
            _ => Action::Empower,
        }
    }
}

fn main() {
    println!("Welcome to scythe statistics!");

    let mut nation = Nation::new("Rusviet", "Joey");

    while !nation.has_won() {
        let choice = rand::random::<Action>();

        //println!("Turn: {} == Action: {:?}", nation.turns, choice);
        //println!("{nation:?}");

        //let mut input = String::new();
        //io::stdin()
        //.read_line(&mut input)
        //.expect("Failed to read the line!");

        match choice {
            Action::Move => {
                match rand::thread_rng().gen_range(0..2) {
                    0 => {
                        nation.fields += 1;
                    }
                    1 => {
                        nation.coins += 2;
                    }
                    _ => panic!("Invalid execution path in trade action"),
                }

                nation.turns += 1;

                if !nation.evolutions.is_max() && nation.resources.food >= 3 {
                    nation.resources.food -= 3;
                    nation.evolutions.add(1);
                }
            }
            Action::Produce => {
                nation.resources.add(rand::random::<Resource>(), 2);

                nation.turns += 1;

                if !nation.mechs.is_max() && nation.resources.metal >= 3 {
                    nation.resources.metal -= 3;
                    nation.mechs.add(1);
                }
            }
            Action::Trade => {
                if nation.coins <= 0 {
                    continue;
                }

                nation.coins -= 1;

                match rand::thread_rng().gen_range(0..2) {
                    0 => {
                        nation.popularity.add(1);
                    }
                    1 => {
                        nation.resources.add(rand::random::<Resource>(), 1);
                        nation.resources.add(rand::random::<Resource>(), 1);
                    }
                    _ => panic!("Invalid execution path in trade action"),
                }

                nation.turns += 1;

                if !nation.buildings.is_max() && nation.resources.wood >= 3 {
                    nation.resources.wood -= 3;
                    nation.buildings.add(1);
                }
            }
            Action::Empower => {
                nation.military.set(nation.military.power + 2);

                nation.turns += 1;

                if !nation.recruits.is_max() && nation.resources.oil >= 3 {
                    nation.resources.oil -= 3;
                    nation.recruits.add(1);
                }
            }
        }
    }

    println!(
        "Congratulations! You have won the game in {} turns!",
        nation.turns
    );
    println!("{nation:?}");
}
