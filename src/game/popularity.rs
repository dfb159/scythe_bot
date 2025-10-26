#[derive(Debug, Clone, Copy)]
pub struct PopularityState {
    pub popularity: u8,
    pub star: bool,
}

pub const MAX: u8 = 18;

impl PopularityState {
    pub fn new(popularity: u8) -> PopularityState {
        PopularityState {
            popularity,
            star: false,
        }
    }

    pub fn change(&mut self, popularity: i8) {
        let new_popularity = self.popularity.saturating_add_signed(popularity);
        self.set(new_popularity);
    }

    pub fn add(&mut self, popularity: u8) {
        let new_popularity = self.popularity.saturating_add(popularity);
        self.set(new_popularity);
    }

    pub fn sub(&mut self, popularity: u8) {
        let new_popularity = self.popularity.saturating_sub(popularity);
        self.set(new_popularity);
    }

    pub fn set(&mut self, popularity: u8) {
        self.popularity = popularity;
        if self.popularity >= MAX {
            self.popularity = MAX;
            self.star = true;
        }
    }

    pub fn star_multiplier(&self) -> u8 {
        if self.popularity < 7 {
            3
        } else if self.popularity < 14 {
            4
        } else {
            5
        }
    }

    pub fn fields_multiplier(&self) -> u8 {
        if self.popularity < 7 {
            2
        } else if self.popularity < 14 {
            3
        } else {
            4
        }
    }

    pub fn resources_multiplier(&self) -> u8 {
        if self.popularity < 7 {
            1
        } else if self.popularity < 14 {
            2
        } else {
            3
        }
    }
}
