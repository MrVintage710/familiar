use std::{collections::{BTreeMap, VecDeque}, fmt::Debug, marker::PhantomData, sync::{Arc, RwLock, Weak}};

use mlua::{FromLua, Function, Lua, Table, UserData, Value};
use ordermap::OrderMap;
use serde::{ser::Error, Deserialize, Serialize};

use crate::{lua::reference::LuaRef, stat::{query::Query, value::StatValue}};

//==============================================================================================
//        Input
//==============================================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Input(OrderMap<String, InputValue>);

impl Input {
    pub fn new() -> Self {
        Self(OrderMap::new())
    }
    
    pub fn iter(&self) -> ordermap::map::Iter<'_, String, InputValue> {
        self.0.iter()
    }
    
    pub fn iter_mut(&mut self) -> ordermap::map::IterMut<'_, String, InputValue> {
        self.0.iter_mut()
    }
}

impl LuaRef for Input {
    type RefType = LuaInput ;

    fn from_ref(reference : &Self::RefType) -> Option<Arc<RwLock<Self>>> {
        reference.0.upgrade()
    }

    fn make_ref(this : &Arc<RwLock<Self>>) -> Self::RefType {
        LuaInput(Arc::downgrade(this))
    }
}

//==============================================================================================
//        Input Lua
//==============================================================================================

pub struct LuaInput(Weak<RwLock<Input>>);

impl UserData for LuaInput {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("point_buy", |lua : &Lua, this : &mut Self, (name, callback, options) : (String, Function, Option<Table>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            let mut input = Input::default();
            input.as_lua_ref(lua, |_, reference| {
                callback.call(reference)
            })?;
            let values = input.0.into_iter().filter(|(_, input_value)| input_value.is_int()).collect();
            
            let options = options.unwrap_or(lua.create_table()?); 
            let points : i32 = options.get("points").unwrap_or(1); 
            let max : Option<i32> = options.get("max").ok(); 
            
            this.0.insert(name, InputValue::PointBuy { values, points, max });           
            Ok(())
        });
        
        methods.add_method_mut("choice", |lua : &Lua, this : &mut Self, (name, choices, number_of_choices, number_of_selections) : (String, Value, Option<u32>, Option<u32>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            let number_of_selections = number_of_selections.unwrap_or(1);
            let number_of_choices = number_of_choices.unwrap_or(1);
            
            match (Vec::<StatValue>::from_lua(choices.clone(), lua), Query::from_lua(choices.clone(), lua)) {
                (Ok(values), _) => this.0.insert(name, InputValue::Choice {values, number_of_selections, number_of_choices }),
                (_, Ok(query)) => this.0.insert(name, InputValue::ChoiceQuery { query, number_of_selections, number_of_choices }),
                (Err(_), Err(_)) => return Err(mlua::Error::BadArgument { 
                    to: Some("Input:choice".to_string()), 
                    pos: 2, 
                    name: Some("choices".to_string()), 
                    cause: Arc::new(mlua::Error::custom("Argument passed was not the correct type. Must be a query or list.")) 
                }),
            };            
            Ok(())
        });
        
        methods.add_method_mut("section", |lua, this, (name, callback) : (String, Function)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            let mut input = Input::default();
            input.as_lua_ref(lua, |_, reference| {
                callback.call(reference)
            })?;
            let value = InputValue::Section(input.0);
            this.0.insert(name, value);
            Ok(())
        });
        
        methods.add_method_mut("string", |_, this, (name, default) : (String, Option<String>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            this.0.insert(name, InputValue::String{default});
            
            Ok(())
        });
        
        methods.add_method_mut("number", |_, this, (name, default) : (String, Option<f64>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            this.0.insert(name, InputValue::Number{default});
            
            Ok(())
        });
        
        methods.add_method_mut("integer", |_, this, (name, default) : (String, Option<i64>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            this.0.insert(name, InputValue::Int{default});
            
            Ok(())
        });
    }
}

//==============================================================================================
//        Choice
//==============================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputValue {
    PointBuy {
        values : OrderMap<String, InputValue>,
        points : i32,
        max : Option<i32>
    },
    Section(OrderMap<String, InputValue>),
    ChoiceQuery {
        query : Query,
        number_of_selections : u32,
        number_of_choices : u32,
    },
    Choice {
        values : Vec<StatValue>,
        number_of_selections : u32,
        number_of_choices : u32,
    },
    String {
        default : Option<String>
    },
    Number {
        default : Option<f64>
    },
    Int {
        default : Option<i64>
    }
}

impl InputValue {
    /// Returns `true` if the input value is [`PointBuy`].
    ///
    /// [`PointBuy`]: InputValue::PointBuy
    #[must_use]
    pub fn is_point_buy(&self) -> bool {
        matches!(self, Self::PointBuy { .. })
    }
    
    /// Returns `true` if the input value is [`Section`].
    ///
    /// [`Section`]: InputValue::Section
    #[must_use]
    pub fn is_section(&self) -> bool {
        matches!(self, Self::Section(..))
    }

    /// Returns `true` if the input value is [`ChoiceQuery`].
    ///
    /// [`ChoiceQuery`]: InputValue::ChoiceQuery
    #[must_use]
    pub fn is_choice_query(&self) -> bool {
        matches!(self, Self::ChoiceQuery { .. })
    }

    /// Returns `true` if the input value is [`Choice`].
    ///
    /// [`Choice`]: InputValue::Choice
    #[must_use]
    pub fn is_choice(&self) -> bool {
        matches!(self, Self::Choice { .. })
    }

    /// Returns `true` if the input value is [`String`].
    ///
    /// [`String`]: InputValue::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    /// Returns `true` if the input value is [`Number`].
    ///
    /// [`Number`]: InputValue::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number { .. })
    }

    /// Returns `true` if the input value is [`Int`].
    ///
    /// [`Int`]: InputValue::Int
    #[must_use]
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int { .. })
    }
    
    pub fn iter(&self) -> Option<ordermap::map::Iter<'_, String, InputValue>> {
        if let InputValue::Section(map) = self {
            Some(map.iter())
        } else { None }
    }
    
    pub fn iter_mut(&mut self) -> Option<ordermap::map::IterMut<'_, String, InputValue>> {
        if let InputValue::Section(map) = self {
            Some(map.iter_mut())
        } else { None }
    }

}