use std::{cell::RefCell, collections::HashMap, ffi::OsStr, fmt, marker::PhantomData, rc::Rc, str::FromStr};
use mlua::{IntoLua, Lua, Table};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{self, MapAccess, Visitor}};

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

pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = FreError>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = FreError>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
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