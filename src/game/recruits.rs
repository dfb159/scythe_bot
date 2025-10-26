#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Recruit {
    Popularity,
    Power,
    Card,
    Coin,
}

#[derive(Debug, Clone, Copy)]
pub struct RecruitsState {
    pub secondary_military_recruited: bool,
    pub secondary_coin_recruited: bool,
    pub secondary_popularity_recruited: bool,
    pub secondary_card_recruited: bool,

    pub onetime_military_recruited: bool,
    pub onetime_coin_recruited: bool,
    pub onetime_popularity_recruited: bool,
    pub onetime_card_recruited: bool,

    pub star: bool,
}

impl RecruitsState {
    pub fn new() -> RecruitsState {
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

    pub fn recruit(&mut self, secondary: Recruit, onetime: Recruit) {
        match secondary {
            Recruit::Power => {
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
            Recruit::Power => {
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

    pub fn can_recruit(&self, secondary: Recruit, onetime: Recruit) -> bool {
        !self.is_secondary_recruited(secondary) && !self.is_onetime_recruited(onetime)
    }

    pub fn is_secondary_recruited(&self, secondary: Recruit) -> bool {
        match secondary {
            Recruit::Power => self.secondary_military_recruited,
            Recruit::Coin => self.secondary_coin_recruited,
            Recruit::Popularity => self.secondary_popularity_recruited,
            Recruit::Card => self.secondary_card_recruited,
        }
    }

    pub fn is_onetime_recruited(&self, onetime: Recruit) -> bool {
        match onetime {
            Recruit::Power => self.onetime_military_recruited,
            Recruit::Coin => self.onetime_coin_recruited,
            Recruit::Popularity => self.onetime_popularity_recruited,
            Recruit::Card => self.onetime_card_recruited,
        }
    }
}
