use std::{cmp::Ordering, collections::HashMap, fmt::Display, sync::{Arc, RwLock, Weak}};

use mlua::{FromLua, IntoLua, Lua, MetaMethod, UserData, Value, Variadic};
use pak_db::index::{Indices, PakSearchable};
use serde::{de::Visitor, ser::{Error, SerializeMap, SerializeSeq}, Deserialize, Serialize, Serializer};

use crate::{error::{VreError, VreResult}, feature::Feature, lua::reference::LuaRef, stat::{field::{StatBlockField, StatSourceProvider}, value::StatValue}};

//==============================================================================================
//        Stat
//==============================================================================================

#[derive(Debug, Default)]
pub struct StatBlock {
    map : HashMap<String, StatBlockField>,
}

impl StatBlock {
    pub fn new() -> StatBlock {
        StatBlock {
            map: HashMap::new(),
        }
    }
    
    pub fn as_value(&self) -> StatValue {
        StatValue::Record(self.map.iter().map(|(key, value)| (key.clone(), value.get())).collect::<HashMap<String, StatValue>>())
    }
    
    pub fn set_field(&mut self, path : impl StatBlockPath, field : StatBlockField, force : bool) {
        let (head, tail) = path.next();
        if let Some(tail) = tail {
            let value = self.map.get_mut(head);
            if value.is_none() && force {
                self.map.insert(head.to_string(), StatBlockField::Table(StatBlock::new()));
                self.map.get_mut(head).unwrap().as_table_mut().unwrap().set_field(tail, field, force);
            } else {
                let Some(table) = value.unwrap().as_table_mut() else { return };
                table.set_field(tail, field, force);
            }
        } else {
            self.map.insert(head.to_string(), field);
        }
    }
    
    pub fn get_field(&self, path : impl StatBlockPath) -> Option<&StatBlockField> {
        let (head, tail) = path.next();
        if let Some(tail) = tail {
            if let Some(StatBlockField::Table(table)) = self.map.get(head) {
                return table.get_field(tail)
            }
            None
        } else {
            self.get_local_field(head)
        }
    }
    
    pub fn get_local_field(&self, index : &str) -> Option<&StatBlockField> {
        self.map.get(index)
    }
    
    pub fn get_local_field_mut(&mut self, index : &str) -> Option<&mut StatBlockField> {
        self.map.get_mut(index)
    }
    
    pub fn get_field_mut(&mut self, path : impl StatBlockPath) -> Option<&mut StatBlockField> {
        let (head, tail) = path.next();
        if let Some(tail) = tail {
            if let Some(StatBlockField::Table(table)) = self.map.get_mut(head) {
                return table.get_field_mut(tail)
            }
            None
        } else {
            self.map.get_mut(head)
        }
    }
    
    pub fn get_provider(&self, path : impl StatBlockPath) -> Option<&StatBlockField> {
        let (head, tail) = path.next();
        if let Some(tail) = tail {
            if let Some(StatBlockField::Table(table)) = self.map.get(head) {
                return table.get_provider(tail)
            }
            None
        } else {
            self.map.get(head)
        }
    }
    
    pub fn get(&self, path : impl StatBlockPath) -> StatValue {
        let Some(provider) = self.get_provider(path) else {return StatValue::None };
        provider.get()
    }
    
    fn get_flattened_size(&self) -> usize {
        let mut result = self.map.len();
        for field in self.map.iter() {
            let (_, StatBlockField::Table(table)) = field else { continue };
            result += table.get_flattened_size() - 1
        }
        result
    }
    
    fn to_indicies(&self, result : &mut Indices, root : &str) {
        for (key, value) in self.map.iter() {
            let index = if root.is_empty() { key.clone() } else { format!("{root}.{key}") };
            match value {
                StatBlockField::Stat(stat_source) => result.add(index, stat_source.as_pak_value()),
                StatBlockField::Derive(stat_derive) => result.add(index, stat_derive.as_pak_value()),
                StatBlockField::Constant(stat_value) => result.add(index, stat_value.clone()),
                StatBlockField::Table(stat_block) => stat_block.to_indicies(result, &index),
            }
        }
    }
}

impl Serialize for StatBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        fn serialize_flat<S : Serializer>(serializer : &mut S::SerializeSeq, statblock : &StatBlock, current_root: &str) -> Result<(), S::Error> {
            for (key, value) in statblock.map.iter() {
                let index = if current_root.is_empty() { key.clone() } else { format!("{current_root}.{key}") };
                let addr = if let StatBlockField::Derive(derive) = value { derive.get_pointer_usize() } else { 0 };
                match value {
                    StatBlockField::Table(statblock) => serialize_flat::<S>(serializer, statblock, &index)?,
                    _ => serializer.serialize_element(&(index, addr, value))?
                };
            }
            Ok(())
        }
        
        let mut seq = serializer.serialize_seq(Some(self.get_flattened_size()))?;
        serialize_flat::<S>(&mut seq, self, "")?;
        seq.end()
    }
}

impl <'de> Deserialize<'de> for StatBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        
        struct StatBlockVisitor;
        
        impl <'de> Visitor<'de> for StatBlockVisitor {
            type Value = StatBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a StatBlock")
            }
            
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                // This is the final result of the deserialization
                let mut result = StatBlock::new();
                // First we construct a queue of the fields that we need to relink together. This queue should have no constansts or tables.
                // (path, previous_address, field)
                let mut queue = Vec::<(String, usize, StatBlockField)>::new();
                
                // Read all of the fields from the deserializer and add them to the queue above, unless they are a constant.
                while let Ok(Some((key, addr, value))) = seq.next_element() {
                    match value {
                        StatBlockField::Constant(_) => { result.set_field(&key, value, true); },
                        StatBlockField::Table(_) => unreachable!(),
                        _ => { queue.push((key, addr, value)); },
                    };
                }
                
                // Sort the queue into the order that they should be linked.
                queue.sort_by(|(_, a_addr, a), (_, b_addr, b)| {
                    if a.has_derivative(*b_addr) { return Ordering::Less };
                    if b.has_derivative(*a_addr) { return Ordering::Greater };
                    a.derivative_count().cmp(&b.derivative_count()).reverse()
                });
                
                // Go through them one at a time, relink all of their children and add them to the result
                let mut loaded_fields = HashMap::new();
                while let Some((key, addr, mut value)) = queue.pop() {
                    for (_, link) in value.links() {
                        let Some(index) = loaded_fields.get(&link) else { continue };
                        let Some(derivative) = result.get_field_mut(index) else { continue };
                        value.bind(derivative, link);
                    }
                    
                    result.set_field(&key, value, true);
                    loaded_fields.insert(addr, key);
                }
                
                // Done!
                Ok(result)
            }
        }
        
        deserializer.deserialize_seq(StatBlockVisitor)
    }
}

impl IntoLua for StatBlock {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        self.map.into_lua(lua)
    }
}

impl FromLua for StatBlock {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        let Ok(map) = HashMap::<String, StatBlockField>::from_lua(value.clone(), lua) else {
            return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: std::any::type_name::<Self>().to_string(), message: None })
        };
        
        Ok(StatBlock { map })
    }
}

impl From<HashMap<String, StatBlockField>> for StatBlock {
    fn from(map: HashMap<String, StatBlockField>) -> Self {
        StatBlock { map }
    }
}

impl PakSearchable for StatBlock {
    fn get_indices(&self, indices : &mut Indices) {
        self.to_indicies(indices, "");
    }
}

impl Clone for StatBlock {
    fn clone(&self) -> Self {
        /// Recursive flatten function
        fn flatten(values : &mut Vec<(String, usize, StatBlockField)>, statblock : &StatBlock, result : &mut StatBlock, current_root: &str) {
            for (key, value) in statblock.map.iter() {
                let index = if current_root.is_empty() { key.clone() } else { format!("{current_root}.{key}") };
                let addr = if let StatBlockField::Derive(derive) = &value { derive.get_pointer_usize() } else { 0 };
                match value {
                    StatBlockField::Table(statblock) => flatten(values, statblock, result, &index),
                    StatBlockField::Constant(_) => result.set_field(&index, value.clone(), true),
                    _ => values.push((index, addr, value.clone()))
                };
            }
        }
        
        let mut result = StatBlock::new();
        
        let mut values : Vec<(String, usize, StatBlockField)> = vec![];
        flatten(&mut values, self, &mut result, "");
        
        // Sort the queue into the order that they should be linked.
        values.sort_by(|(_, a_addr, a), (_, b_addr, b)| {
            if a.has_derivative(*b_addr) { return Ordering::Less };
            if b.has_derivative(*a_addr) { return Ordering::Greater };
            a.derivative_count().cmp(&b.derivative_count()).reverse()
        });
        
        // Go through them one at a time, relink all of their children and add them to the result
        let mut loaded_fields = HashMap::new();
        while let Some((key, addr, mut value)) = values.pop() {
            for (_, link) in value.links() {
                let Some(index) = loaded_fields.get(&link) else { continue };
                let Some(derivative) = result.get_field_mut(index) else { continue };
                value.bind(derivative, link);
            }
            
            result.set_field(&key, value, true);
            loaded_fields.insert(addr, key);
        }
        
        result
    }
}

impl PartialEq for StatBlock {
    fn eq(&self, other: &Self) -> bool {
        for (key, value) in self.map.iter() {
            let Some(other_value) = self.map.get(key) else { return false };
            if value.get() != other_value.get() { return false }
        }
        true
    }
}

// impl LuaRef for StatBlock {
//     type RefType = LuaStatBlockRef;

//     fn from_ref(reference : &Self::RefType) -> Option<Arc<RwLock<Self>>> {
//         reference.pointer.upgrade()
//     }

//     fn make_ref(this : &Arc<RwLock<Self>>) -> Self::RefType {
//         LuaStatBlockRef {
//             pointer: Arc::downgrade(this),
//             root: String::new(),
//         }
//     }
// }

//==============================================================================================
//        Lua Statblock
//==============================================================================================

// fn stat_block_index(lua : &Lua, statblock : &Arc<RwLock<StatBlock>>, root : String) -> mlua::Result<Value> {
//     let Ok(statblock_ref) = statblock.read() else { return Ok(Value::Nil) };
//     let Some(field) = statblock_ref.get_field(&root) else { return Ok(Value::Nil) };
//     match field {
//         StatBlockField::Constant(value) => value.clone().into_lua(lua),
//         StatBlockField::Stat(_) => LuaStatBlockSource::from_arc(statblock).with_root(root).into_lua(lua),
//         StatBlockField::Derive(_) => LuaStatBlockDerive::from_arc(statblock).with_root(root).into_lua(lua),
//         _ => LuaStatBlockRef::from_arc(statblock).with_root(root).into_lua(lua)
//     }
// }

// fn stat_block_new_index(statblock : &Arc<RwLock<StatBlock>>, root : String, value : StatBlockField) {
//     let Ok(mut statblock_ref) = statblock.write() else { return };
//     statblock_ref.set(&root, value, true);
// }

// fn stat_block_source_get(statblock : &Arc<RwLock<StatBlock>>, root : &str) -> StatValue {
//     let Ok(statblock) = statblock.read() else { return  StatValue::None };
//     statblock.get(root)
// }

// fn stat_block_source_set(lua : &Lua, statblock : &Arc<RwLock<StatBlock>>, root : &str, value : StatValue) {
//     let Ok(mut statblock) = statblock.write() else { return };
//     let Some(field) = statblock.get_field_mut(root) else { return };
//     field.set(lua, value);
// }

// struct LuaStatBlock(Arc<RwLock<StatBlock>>);

// impl LuaStatBlock {
//     pub fn new(statblock : StatBlock) -> Self {
//         LuaStatBlock(Arc::new(RwLock::new(statblock)))
//     }
    
//     pub fn into_inner(self) -> mlua::Result<StatBlock> {
//         let Ok(statblock) = Arc::try_unwrap(self.0) else { return Err(mlua::Error::custom("Unable to unwrap the statblock."))};
//         let Ok(statblock) = statblock.into_inner() else { return Err(mlua::Error::custom("Unable to unwrap the statblock."))};
//         Ok(statblock)
//     }
// }

// impl UserData for LuaStatBlock {
//     fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
//         methods.add_meta_method(MetaMethod::Index, |lua, this, key : String| {
//             stat_block_index(lua, &this.0, key)
//         });
        
//         methods.add_meta_method(MetaMethod::NewIndex, |_, this, (key, value) : (String, StatBlockField)| {
//             stat_block_new_index(&this.0, key, value);
//             Ok(())
//         });
//     }
// }

// pub(crate) struct LuaStatBlockRef {
//     pointer : Weak<RwLock<StatBlock>>,
//     root : String
// }

// impl LuaStatBlockRef {
//     pub fn from_arc(statblock : &Arc<RwLock<StatBlock>>) -> LuaStatBlockRef {
//         LuaStatBlockRef {
//             pointer: Arc::downgrade(statblock),
//             root: String::new(),
//         }
//     }
    
//     pub fn with_root(mut self, root : impl ToString) -> Self {
//         self.root = root.to_string();
//         self
//     }
// }

// impl UserData for LuaStatBlockRef {
//     fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
//         methods.add_meta_method_mut(MetaMethod::Index, |lua, this, key : String| {
//             let Some(statblock) = this.pointer.upgrade() else { return Ok(Value::Nil) };
//             let index = if this.root.is_empty() { key.clone() } else { format!("{}.{key}", this.root) };
//             stat_block_index(lua, &statblock, index)
//         });
        
//         methods.add_meta_method(MetaMethod::NewIndex, |_, this, (key, value) : (String, StatBlockField)| {
//             let Some(statblock) = this.pointer.upgrade() else { return Ok(Value::Nil) };
//             let index = if this.root.is_empty() { key.clone() } else { format!("{}.{key}", this.root) };
//             stat_block_new_index(&statblock, index, value);
//             Ok(Value::Nil)
//         });
//     }
// }

// struct LuaStatBlockDerive {
//     pointer : Weak<RwLock<StatBlock>>,
//     root : String
// }

// impl LuaStatBlockDerive {
//     pub fn from_arc(statblock : &Arc<RwLock<StatBlock>>) -> Self {
//         Self {
//             pointer: Arc::downgrade(statblock),
//             root: String::new(),
//         }
//     }
    
//     pub fn with_root(mut self, root : impl ToString) -> Self {
//         self.root = root.to_string();
//         self
//     }
// }

// impl UserData for LuaStatBlockDerive {
//     fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
//         methods.add_method("get", |lua, this, ()| {
//             let Some(statblock) = this.pointer.upgrade() else { return Ok(Value::Nil) };
//             stat_block_source_get(&statblock, &this.root).into_lua(lua)
//         });
//     }
// }

// struct LuaStatBlockSource {
//     pointer : Weak<RwLock<StatBlock>>,
//     root : String
// }

// impl LuaStatBlockSource {
//     pub fn from_arc(statblock : &Arc<RwLock<StatBlock>>) -> Self {
//         Self {
//             pointer: Arc::downgrade(statblock),
//             root: String::new(),
//         }
//     }
    
//     pub fn with_root(mut self, root : impl ToString) -> Self {
//         self.root = root.to_string();
//         self
//     }
// }

// impl UserData for LuaStatBlockSource {
//     fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
//         methods.add_method("get", |lua, this, ()| {
//             let Some(statblock) = this.pointer.upgrade() else { return Ok(Value::Nil) };
//             stat_block_source_get(&statblock, &this.root).into_lua(lua)
//         });
        
//         methods.add_method("set", |lua, this, field : StatValue| {
//             let Some(statblock) = this.pointer.upgrade() else { return Ok(()) };
//             stat_block_source_set(lua, &statblock, &this.root, field);
            
//             Ok(())
//         });
//     }
// }

//==============================================================================================
//        StatBlockPath
//==============================================================================================

pub trait StatBlockPath : Display {
    fn as_str(&self) -> &str;

    fn in_parts(&self) -> Vec<&str>;
    
    fn next<'a>(&'a self) -> (&'a str, Option<&'a str>);
    
    fn is_empty(&self) -> bool;
    
    fn append_path(&self, other : impl StatBlockPath) -> String {
        if self.is_empty() { format!("{other}") } else { format!("{self}.{other}") }
    }
}

impl StatBlockPath for &str {
    fn as_str(&self) -> &str {
        self
    }

    fn in_parts(&self) -> Vec<&str> {
        self.split('.').collect()
    }

    fn next(&self) -> (&str, Option<&str>) {
        let Some(index) = self.find(".") else { return (self, None) };
        let first = &self[..index];
        let rest = &self[(index + 1)..];
        (first, Some(rest))
    }

    fn is_empty(&self) -> bool {
        str::is_empty(&self)
    }
}

impl StatBlockPath for &String {
    fn as_str(&self) -> &str {
        self
    }

    fn in_parts(&self) -> Vec<&str> {
        self.split('.').collect()
    }

    fn next(&self) -> (&str, Option<&str>) {
        let Some(index) = self.find(".") else { return (self, None) };
        let first = &self[..index];
        let rest = &self[(index + 1)..];
        (first, Some(rest))
    }

    fn is_empty(&self) -> bool {
        String::is_empty(&self)
    }
}

