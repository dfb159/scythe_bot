use std::rc::Rc;

use crate::game::board::Field;

#[derive(Debug, Clone)]
pub struct CharacterEntity {
    pub location: Rc<Field>,
}
