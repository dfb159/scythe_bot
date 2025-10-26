use std::u8;

#[derive(Debug, Clone, Copy)]
pub struct MilitaryState {
    pub power: u8,
    pub star: bool,
}

pub const MAX: u8 = 16;

impl MilitaryState {
    pub fn new(power: u8) -> MilitaryState {
        MilitaryState { power, star: false }
    }

    pub fn change(&mut self, power: i8) {
        let new_power = self.power.saturating_add_signed(power);
        self.set(new_power);
    }

    pub fn add(&mut self, power: u8) {
        let new_power = self.power.saturating_add(power);
        self.set(new_power);
    }

    pub fn sub(&mut self, power: u8) {
        let new_power = self.power.saturating_sub(power);
        self.set(new_power);
    }

    pub fn set(&mut self, power: u8) {
        self.power = power;
        if self.power >= MAX {
            self.power = MAX;
            self.star = true;
        }
    }
}
