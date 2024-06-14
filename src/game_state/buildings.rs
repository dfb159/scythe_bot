use super::resources::Resource;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Building {
    Mine,
    Mill,
    Armory,
    Monument,
}

#[derive(Debug)]
pub(crate) struct BuildingsState {
    pub(crate) mine_build: bool,
    pub(crate) mill_build: bool,
    pub(crate) armory_build: bool,
    pub(crate) monument_build: bool,

    pub(crate) mill_location: Option<Resource>,

    pub(crate) star: bool,
}

impl BuildingsState {
    pub(crate) fn new() -> BuildingsState {
        BuildingsState {
            mine_build: false,
            mill_build: false,
            armory_build: false,
            monument_build: false,

            mill_location: None,

            star: false,
        }
    }

    pub(crate) fn build(&mut self, building: Building) {
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

    pub(crate) fn can_build(&self, building: Building) -> bool {
        match building {
            Building::Mine => !self.mine_build,
            Building::Mill => !self.mill_build,
            Building::Armory => !self.armory_build,
            Building::Monument => !self.monument_build,
        }
    }
}
