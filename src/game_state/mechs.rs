#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Mech {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug)]
pub(crate) struct MechsState {
    pub(crate) first_deployed: bool,
    pub(crate) second_deployed: bool,
    pub(crate) third_deployed: bool,
    pub(crate) fourth_deployed: bool,
    pub(crate) star: bool,
}

impl MechsState {
    pub(crate) fn new() -> MechsState {
        MechsState {
            first_deployed: false,
            second_deployed: false,
            third_deployed: false,
            fourth_deployed: false,
            star: false,
        }
    }

    pub(crate) fn deploy(&mut self, mech: Mech) {
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

    pub(crate) fn can_deploy(&self, mech: Mech) -> bool {
        match mech {
            Mech::First => !self.first_deployed,
            Mech::Second => !self.second_deployed,
            Mech::Third => !self.third_deployed,
            Mech::Fourth => !self.fourth_deployed,
        }
    }
}
