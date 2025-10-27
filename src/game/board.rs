use std::{collections::HashMap, ops::Add, rc::Rc};

use crate::{
    game::Tile,
    template::{BoardTemplate, Position},
};

#[derive(Debug, Clone)]
pub struct Board {
    pub fields: HashMap<Position, Rc<Field>>,
    pub rivers: Vec<(Rc<Field>, Rc<Field>)>,
}

impl Board {
    pub fn from_template<const F: usize, const R: usize, const P: usize>(
        template: &BoardTemplate<F, R, P>,
    ) -> Self {
        let mut fields = HashMap::with_capacity(F+P);
        for f in template.fields.iter() {
            let new_field = Rc::new(Field {
                encounter_token: f.explorer_token,
                tunnelable: f.tunnelable,
                tile: f.tile,
                position: f.position,
                resources: ResourceField {
                    wood: 0,
                    metal: 0,
                    oil: 0,
                    food: 0,
                },
            });
            if fields.insert(f.position, new_field).is_some() {
                panic!("Board Field is defined twice!")
            }
        }
        for home in template.starting_locations.iter() {
            let new_field = Rc::new(Field {
                encounter_token: false,
                tunnelable: false,
                tile: Tile::Home,
                position: home.position,
                resources: ResourceField {
                    wood: 0,
                    metal: 0,
                    oil: 0,
                    food: 0,
                },
            });
            if fields.insert(home.position, new_field).is_some() {
                panic!("Board Field is defined twice!")
            }
        }

        let mut rivers = Vec::with_capacity(R);
        for (p1, p2) in template.rivers.iter() {
            if let Some(f1) = fields.get(p1) {
                if let Some(f2) = fields.get(p2) {
                    rivers.push((f1.clone(), f2.clone()));
                }
            }
        }

        Board {
            fields: fields,
            rivers: rivers,
        }
    }

    pub fn get_field(&self, position: &Position) -> Option<Rc<Field>> {
        self.fields.get(position).cloned()
    }

    pub fn is_river(&self, from: &Field, to: &Field) -> bool {
        // TODO maybe need to pass &Rc<Field> here
        self.rivers
            .iter()
            .any(|(f1, f2)| (**f1 == *from && **f2 == *to) || (**f2 == *from && **f1 == *to))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    pub encounter_token: bool,
    pub tunnelable: bool,
    pub tile: Tile,
    pub position: Position,
    pub resources: ResourceField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceField {
    pub wood: u32,
    pub metal: u32,
    pub oil: u32,
    pub food: u32,
}

impl ResourceField {
    pub fn total(&self) -> u32 {
        self.wood.saturating_add(self.metal).saturating_add(self.oil).saturating_add(self.food)
    }
}

impl Add for ResourceField {
    type Output = ResourceField;

    fn add(self, rhs: Self) -> Self::Output {
        ResourceField {
            wood: self.wood.saturating_add(rhs.wood),
            metal: self.metal.saturating_add(rhs.metal),
            oil: self.oil.saturating_add(rhs.oil),
            food: self.food.saturating_add(rhs.food),
        }
    }
}
