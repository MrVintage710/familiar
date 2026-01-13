use std::{collections::{BTreeMap, VecDeque}, fmt::Debug, marker::PhantomData, sync::{Arc, RwLock, Weak}};

use mlua::{FromLua, Function, Lua, UserData, Value};
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
    
    // pub fn set_choice(&mut self, name : &str, index : usize) {
    //     let Some(value) = self.1.get_mut(name) else { return };
    //     if !value.is_choice() { return }
    //     if value.is_complete() {
    //         value.pop();
    //     }
    //     value.choose(index);
    // }
    
    // pub fn set_string(&mut self, name : &str, string : &str) {
    //     let Some(value) = self.1.get_mut(name) else { return };
    //     if !value.is_string() { return }
    //     value.set_string(string.to_string());
    // }
    
    // pub fn set_number(&mut self, name : &str, number : f64) {
    //     let Some(value) = self.1.get_mut(name) else { return };
    //     if !value.is_string() { return }
    //     value.set_number(number);
    // }
    
    // pub fn set_int(&mut self, name : &str, int : i64) {
    //     let Some(value) = self.1.get_mut(name) else { ret0.2.3urn };
    //     if !value.is_string() { return }
    //     value.set_int(int);
    // }
    
    // pub fn choice_count(&self) -> usize {
    //     self.1.len()
    // }
    
    // pub fn is_complete(&self) -> bool {
    //     self.1.iter().all(|(_, entry)| entry.is_complete())
    // }
    
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
    pub fn is_choice(&self) -> bool {
        matches!(self, InputValue::Choice { .. } | InputValue::ChoiceQuery { .. })
    }
    
    pub fn is_section(&self) -> bool {
        matches!(self, InputValue::Section(..))
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, InputValue::String{..})
    }
    
    pub fn is_number(&self) -> bool {
        matches!(self, InputValue::Number{..})
    }
    
    pub fn is_int(&self) -> bool {
        matches!(self, InputValue::Int{..})
    }
    
    // pub fn choices_made(&self) -> usize {
    //     match self {
    //         InputValue::ChoiceQuery { choices, .. } => choices.len(),
    //         InputValue::Choice { choices, .. } => choices.len(),
    //         InputValue::String(value, ..) => if value.is_some() { 1 } else { 0 },
    //         InputValue::Number(value, ..) => if value.is_some() { 1 } else { 0 },
    //         InputValue::Int(value, ..) => if value.is_some() { 1 } else { 0 },
    //         InputValue::Section(inputs) => inputs.iter().map(|i| i.1.choices_made()).sum()
    //     }
    // }
    
    // pub fn max_choices(&self) -> usize {
    //     match self {
    //         InputValue::ChoiceQuery { number_of_choices, .. } => *number_of_choices as usize,
    //         InputValue::Choice { number_of_choices, .. } => *number_of_choices as usize,
    //         InputValue::Section(inputs) => inputs.iter().map(|i| i.1.max_choices()).sum(),
    //         _ => 1
    //     }
    // }
    
    // pub fn is_complete(&self) -> bool {
    //     match self {
    //         InputValue::Section(values) => values.iter().all(|i| i.1.is_complete()),
    //         InputValue::ChoiceQuery { number_of_choices, choices, .. } => choices.len() >= *number_of_choices as usize,
    //         InputValue::Choice { number_of_choices, choices, .. } => choices.len() >= *number_of_choices as usize,
    //         InputValue::String(value) => value.is_some(),
    //         InputValue::Number(value) => value.is_some(),
    //         InputValue::Int(value) => value.is_some(),
    //     }
    // }
    
    // pub fn pop(&mut self) {
    //     match self {
    //         InputValue::ChoiceQuery { choices, .. } => { choices.pop_front(); },
    //         InputValue::Choice { choices, .. } => { choices.pop_front(); },
    //         InputValue::String(value, ..) => { *value = None },
    //         InputValue::Number(value, ..) => { *value = None},
    //         InputValue::Int(value, ..) => { *value = None },
    //         _ => {}
    //     };
    // }
    
    // pub fn choose(&mut self, index : usize) {
    //     match self {
    //         InputValue::ChoiceQuery { choices, .. } => { choices.push_back(index as u32); },
    //         InputValue::Choice { choices, .. } => { choices.push_back(index as u32); },
    //         _ => {}
    //     };
    // }
    
    // pub fn set_string(&mut self, string : String) {
    //     match self {
    //         Self::String(value, ..) => { *value = Some(string); },
    //         _ => {}
    //     }
    // }
    
    // pub fn set_number(&mut self, number : f64) {
    //     match self {
    //         Self::Number(value, ..) => { *value = Some(number); },
    //         _ => {}
    //     }
    // }
    
    // pub fn set_int(&mut self, int : i64) {
    //     match self {
    //         Self::Int(value, ..) => { *value = Some(int); },
    //         _ => {}
    //     }
    // }
    
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