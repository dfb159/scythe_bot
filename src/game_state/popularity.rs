#[derive(Debug)]
pub(crate) struct PopularityState {
    pub(crate) popularity: i32,
    pub(crate) star: bool,
}

impl PopularityState {
    pub(crate) fn new(popularity: i32) -> PopularityState {
        PopularityState {
            popularity,
            star: false,
        }
    }

    pub(crate) fn add(&mut self, popularity: i32) {
        self.set(self.popularity + popularity);
    }

    pub(crate) fn set(&mut self, popularity: i32) {
        self.popularity = popularity;
        if self.popularity >= 18 {
            self.popularity = 18;
            self.star = true;
        }
        if self.popularity < 0 {
            self.popularity = 0;
        }
    }

    pub(crate) fn star_multiplier(&self) -> i32 {
        if self.popularity < 7 {
            3
        } else if self.popularity < 14 {
            4
        } else {
            5
        }
    }

    pub(crate) fn fields_multiplier(&self) -> i32 {
        if self.popularity < 7 {
            2
        } else if self.popularity < 14 {
            3
        } else {
            4
        }
    }

    pub(crate) fn resources_multiplier(&self) -> i32 {
        if self.popularity < 7 {
            1
        } else if self.popularity < 14 {
            2
        } else {
            3
        }
    }
}
