#[derive(Debug)]
pub(crate) struct MilitaryState {
    pub(crate) power: i32,
    pub(crate) star: bool,
}

impl MilitaryState {
    pub(crate) fn new(power: i32) -> MilitaryState {
        MilitaryState { power, star: false }
    }

    pub(crate) fn add(&mut self, power: i32) {
        self.set(self.power + power);
    }

    pub(crate) fn set(&mut self, power: i32) {
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
