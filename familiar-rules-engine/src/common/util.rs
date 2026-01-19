use std::{cell::RefCell, collections::HashMap, ffi::OsStr, rc::Rc};
use mlua::{IntoLua, Lua, Table};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{FreError, FreResult};


//==============================================================================================
//        Serde Helpers
//==============================================================================================

pub fn serialize_wrapped<T, S>(value : &Rc<RefCell<T>>, serializer : S) -> Result<S::Ok, S::Error> where S: Serializer, T : Clone + Serialize  {
    Ok(value.borrow().serialize(serializer)?)
}

pub fn deserialize_wrapped<'de, T,D>(deserializer : D) -> Result<Rc<RefCell<T>>, D::Error> where D: Deserializer<'de>, T : Clone + Deserialize<'de> {
    Ok(Rc::new(RefCell::new(T::deserialize(deserializer)?)))
}

//==============================================================================================
//        Lua Helpers
//==============================================================================================

pub fn map_to_lua<A, B>(lua : &Lua, map : HashMap<A, B>) -> mlua::Result<Table> where A : IntoLua, B : IntoLua {
    let table = map.into_iter().fold(lua.create_table()?, |table, (key, value)| { table.set(key, value); table });
    Ok(table)
}

//==============================================================================================
//        Mime Type
//==============================================================================================

pub fn get_mime_type(extention : &OsStr) -> FreResult<String> {
    if extention == "png" { return Ok("image/png".to_string()) }
    if extention == "jpeg" || extention == "jpg" { return Ok("image/jpeg".to_string()) }
    if extention == "webp" { return Ok("image/webp".to_string()) }
    Err(FreError::MissingMimeType(extention.to_str().unwrap().to_string()))
}

//==============================================================================================
//        General Helpers
//==============================================================================================

pub trait FlattenMap<K, V> {
    type Map<A, B>;
    
    fn flatten_mut(&mut self) -> Self::Map<K, &mut V>;
}

impl <V> FlattenMap<String, V> for HashMap<String, V> {
    type Map<A, B> = HashMap<A, B>;

    fn flatten_mut(&mut self) -> Self::Map<String, &mut V> {
        let flat = HashMap::new();
        
        flat
    }
}

fn flaten_hm_r<V>(this : &mut HashMap<String, V>, flat : HashMap<String, &mut V>, root : &str) {
    for (key, value) in this.iter_mut() {
        let index = if root.is_empty() { key.clone() } else { format!("{root}.{key}") };
    }
}