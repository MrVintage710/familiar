
use mlua::{FromLua, IntoLua, Lua, UserDataRefMut, Value::{self}};
use serde::{Deserialize, Serialize};
use crate::stat::{derive::StatDerive, source::StatSource, statblock::StatBlock, value::StatValue};

//==============================================================================================
//        Proivder
//==============================================================================================

pub trait StatSourceProvider {
    fn get(&self) -> StatValue;
    
    fn push_derive(&mut self, derive : &StatDerive);
}

//==============================================================================================
//        StatProvider
//==============================================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatBlockField {
    Stat(StatSource),
    Derive(StatDerive),
    Constant(StatValue),
    Table(StatBlock)
}

impl From<StatSource> for StatBlockField {
    fn from(value: StatSource) -> Self {
        StatBlockField::Stat(value)
    }
}

impl From<StatDerive> for StatBlockField {
    fn from(value: StatDerive) -> Self {
        StatBlockField::Derive(value)
    }
}

impl StatBlockField {
    pub fn set(&mut self, lua : &Lua, value : impl Into<StatValue>) {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.set(lua, value),
            _ => {}
        }
    }
    
    pub fn is_stat(&self) -> bool {
        matches!(self, StatBlockField::Stat(_))
    }
    
    pub fn is_derive(&self) -> bool {
        matches!(self, StatBlockField::Derive(_))
    }
    
    pub fn is_constant(&self) -> bool {
        matches!(self, StatBlockField::Constant(_))
    }
    
    pub fn is_table(&self) -> bool {
        matches!(self, StatBlockField::Table(_))
    }
    
    pub fn as_stat(&self) -> Option<&StatSource> {
        match self {
            StatBlockField::Stat(stat) => Some(stat),
            _ => None
        }
    }
    
    pub fn as_derive(&self) -> Option<&StatDerive> {
        match self {
            StatBlockField::Derive(derive) => Some(derive),
            _ => None
        }
    }
    
    pub fn as_derive_mut(&mut self) -> Option<&mut StatDerive> {
        match self {
            StatBlockField::Derive(derive) => Some(derive),
            _ => None
        }
    }
    
    pub fn as_table_mut(&mut self) -> Option<&mut StatBlock> {
        match self {
            StatBlockField::Table(table) => Some(table),
            _ => None
        }
    }
    
    pub(crate) fn addr(&self) -> usize {
        match self {
            Self::Derive(derive) => derive.get_pointer_usize(),
            _ => 0
        }
    }
    
    pub(crate) fn has_derivative(&self, addr : usize) -> bool {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.has_derivative(addr),
            StatBlockField::Derive(derive) => derive.has_derivative(addr),
            _ => false
        }
    }
    
    pub fn links(&self) -> Vec<(usize, usize)> {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.links(),
            StatBlockField::Derive(stat_derive) => stat_derive.links(),
            _ => vec![],
        }
    }
    
    pub fn derivative_count(&self) -> usize {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.derivative_count(),
            StatBlockField::Derive(derive) => derive.derivative_count(),
            _ => 0
        }
    }
    
    pub(crate) fn bind(&mut self, derivative : &mut StatBlockField, addr : usize) {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.bind(derivative, addr),
            StatBlockField::Derive(stat_derive) => stat_derive.bind(derivative, addr),
            _ => {}
        }
    }
}

impl StatSourceProvider for StatBlockField {
    fn get(&self) -> StatValue {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.get(),
            StatBlockField::Derive(stat_derive) => stat_derive.get(),
            StatBlockField::Constant(value) => value.clone(),
            StatBlockField::Table(table) => table.as_value(),
        }
    }

    fn push_derive(&mut self, derive : &StatDerive) {
        match self {
            StatBlockField::Stat(stat_source) => stat_source.push_derive(derive),
            StatBlockField::Derive(stat_derive) => stat_derive.push_derive(derive),
            _ => {}
        }
    }
}

impl FromLua for StatBlockField {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            Value::UserData(userdata) => {
                if userdata.is::<StatSource>() {
                    Ok(Self::Stat(userdata.take()?))
                } else if userdata.is::<StatDerive>() {
                    Ok(Self::Derive(userdata.take()?))
                } else {
                    Err(mlua::Error::FromLuaConversionError { from: Value::UserData(userdata).type_name(), to: "StatProvider".to_string(), message: None })
                }
            },
            Value::Table(table) => {
                let statblock = StatBlock::from_lua(Value::Table(table), lua)?;
                Ok(StatBlockField::Table(statblock))
            }
            _ => Ok(Self::Constant(StatValue::from_lua(value, lua)?))
        }
    }
}

impl IntoLua for StatBlockField {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        match self {
            StatBlockField::Stat(stat_source) => Ok(Value::UserData(lua.create_userdata(stat_source)?)),
            StatBlockField::Derive(stat_derive) => Ok(Value::UserData(lua.create_userdata(stat_derive)?)),
            StatBlockField::Constant(stat_value) => stat_value.into_lua(lua),
            StatBlockField::Table(table) => table.into_lua(lua)
        }
    }
}

//==============================================================================================
//        StatProviderRef
//==============================================================================================

pub enum StatProviderRef {
    Stat(UserDataRefMut<StatSource>),
    Derive(UserDataRefMut<StatDerive>),
    Constant(StatValue)
}

impl StatProviderRef {
    pub fn set(&mut self, lua : &Lua, value : impl Into<StatValue>) {
        match self {
            StatProviderRef::Stat(stat_source) => stat_source.set(lua, value),
            _ => {}
        }
    }
    
    pub fn is_stat(&self) -> bool {
        matches!(self, StatProviderRef::Stat(_))
    }
    
    pub fn is_derive(&self) -> bool {
        matches!(self, StatProviderRef::Derive(_))
    }
    
    pub fn is_constant(&self) -> bool {
        matches!(self, StatProviderRef::Constant(_))
    }
    
    pub fn as_derive(&self) -> Option<&StatDerive> {
        match self {
            StatProviderRef::Derive(derive) => Some(derive),
            _ => None
        }
    }
}

impl From<UserDataRefMut<StatSource>> for StatProviderRef {
    fn from(value: UserDataRefMut<StatSource>) -> Self {
        StatProviderRef::Stat(value)
    }
}

impl From<UserDataRefMut<StatDerive>> for StatProviderRef {
    fn from(value: UserDataRefMut<StatDerive>) -> Self {
        StatProviderRef::Derive(value)
    }
}

impl FromLua for StatProviderRef {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let Some(userdata) = value.as_userdata() else { return Ok(StatProviderRef::Constant(StatValue::from_lua(value, lua)?))};
        if userdata.is::<StatSource>() {
            Ok(StatProviderRef::Stat(userdata.borrow_mut::<StatSource>()?))
        } else if userdata.is::<StatDerive>() {
            Ok(StatProviderRef::Derive(userdata.borrow_mut::<StatDerive>()?))
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "StatProviderRef".to_string(), message: None })
        }
    }
}

impl StatSourceProvider for StatProviderRef {
    fn get(&self) -> StatValue {
        match self {
            StatProviderRef::Stat(stat_source) => stat_source.get(),
            StatProviderRef::Derive(stat_derive) => stat_derive.get(),
            StatProviderRef::Constant(value) => value.clone()
        }
    }

    fn push_derive(&mut self, derive : &StatDerive) {
        match self {
            StatProviderRef::Stat(stat_source) => stat_source.push_derive(derive),
            StatProviderRef::Derive(stat_derive) => stat_derive.push_derive(derive),
            _ => {},
        }
    }
}