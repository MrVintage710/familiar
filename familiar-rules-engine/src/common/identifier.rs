use mlua::UserData;
use pak_db::{group::DeserializeGroup, index::{PakIndexIdentifier, PakSearchable}, query::PakQueryExpression};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//==============================================================================================
//        Identifier
//==============================================================================================

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
    rulebook: Option<Uuid>, 
    uuid : Uuid,
    type_name : String,
}

impl Identifier {
    pub fn new<T>(uuid: Uuid) -> Self {
        Self { rulebook : None, uuid, type_name : std::any::type_name::<T>().to_string() }
    }

    pub fn from_rulebook(mut self, rulebook: Uuid) -> Self {
        self.rulebook = Some(rulebook);
        self
    }
    
    pub fn set_rulebook(&mut self, rulebook : Uuid) {
        self.rulebook = Some(rulebook);
    }
    
    pub fn is_type<T>(&self) -> bool {
        self.type_name == std::any::type_name::<T>()
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl UserData for Identifier {}

impl PakSearchable for Identifier {
    fn get_indices(&self, indices : &mut pak_db::index::Indices) {
        indices.add("rulebook", self.rulebook.clone());
        indices.add("id", self.uuid.clone());
        indices.add("type", self.type_name.clone());
    }
}

impl <T : DeserializeGroup + 'static> PakQueryExpression<T> for Identifier {
    fn execute(&self, pak : &pak_db::Pak) -> pak_db::error::PakResult<ordermap::OrderSet<pak_db::pointer::PakPointer>> {
        let query = "rulebook".equals(self.rulebook.clone()) & "id".equals(self.uuid.clone()) & "type_name".equals(self.type_name.clone());
        PakQueryExpression::<T>::execute(&query, pak)
    }
}