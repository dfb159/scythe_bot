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
    pub(crate) mine_built: bool,
    pub(crate) mill_built: bool,
    pub(crate) armory_built: bool,
    pub(crate) monument_built: bool,

    pub(crate) mill_location: Option<Resource>,

    pub(crate) star: bool,
}

impl BuildingsState {
    pub(crate) fn new() -> BuildingsState {
        BuildingsState {
            mine_built: false,
            mill_built: false,
            armory_built: false,
            monument_built: false,

            mill_location: None,

            star: false,
        }
    }

    pub(crate) fn built(&mut self, building: Building) {
        match building {
            Building::Mine => {
                self.mine_built = true;
            }
            Building::Mill => {
                self.mill_built = true;
            }
            Building::Armory => {
                self.armory_built = true;
            }
            Building::Monument => {
                self.monument_built = true;
            }
        }
        if self.mine_built && self.mill_built && self.armory_built && self.monument_built {
            self.star = true;
        }
    }

    pub(crate) fn can_build(&self, building: Building) -> bool {
        match building {
            Building::Mine => !self.mine_built,
            Building::Mill => !self.mill_built,
            Building::Armory => !self.armory_built,
            Building::Monument => !self.monument_built,
        }
    }
}
