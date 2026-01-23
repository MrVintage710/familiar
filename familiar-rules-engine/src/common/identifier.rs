use mlua::UserData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//==============================================================================================
//        Identifier
//==============================================================================================

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
    rulebook: Option<Uuid>, 
    id : Uuid
}

impl Identifier {
    pub fn new(id: Uuid) -> Self {
        Self { rulebook : None, id }
    }

    pub fn from_rulebook(mut self, rulebook: Uuid) -> Self {
        self.rulebook = Some(rulebook);
        self
    }
    
    pub fn set_rulebook(&mut self, rulebook : Uuid) {
        self.rulebook = Some(rulebook);
    }
}

impl UserData for Identifier {}

