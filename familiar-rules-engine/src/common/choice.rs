use std::{collections::VecDeque, fmt::Debug, sync::{Arc, RwLock, Weak}};

use mlua::{FromLua, Lua, UserData, Value};
use serde::{ser::Error, Deserialize, Serialize};

use crate::{lua::reference::LuaRef, stat::{query::Query, value::StatValue}};

//==============================================================================================
//        Input
//==============================================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Input(Vec<InputValue>);

impl Input {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    
    pub fn set_choice(&mut self, name : &str, index : usize) {
        let Some(value) = self.0.iter_mut().find(|value| value.get_name() == name) else { return };
        if !value.is_choice() { return }
        if value.is_complete() {
            value.pop();
        }
        value.choose(index);
    }
    
    pub fn set_string(&mut self, name : &str, string : &str) {
        let Some(value) = self.0.iter_mut().find(|value| value.get_name() == name) else { return };
        if !value.is_string() { return }
        value.set_string(string.to_string());
    }
    
    pub fn set_number(&mut self, name : &str, number : f64) {
        let Some(value) = self.0.iter_mut().find(|value| value.get_name() == name) else { return };
        if !value.is_string() { return }
        value.set_number(number);
    }
    
    pub fn set_int(&mut self, name : &str, int : i64) {
        let Some(value) = self.0.iter_mut().find(|value| value.get_name() == name) else { return };
        if !value.is_string() { return }
        value.set_int(int);
    }
    
    pub fn choice_count(&self) -> usize {
        self.0.len()
    }
    
    pub fn is_complete(&self) -> bool {
        self.0.iter().all(|entry| entry.is_complete())
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
                (Ok(values), _) => this.0.push(InputValue::Choice { name, values, number_of_selections, number_of_choices, choices: VecDeque::new() }),
                (_, Ok(query)) => this.0.push(InputValue::ChoiceQuery { name, query, number_of_selections, number_of_choices, choices: VecDeque::new() }),
                (Err(_), Err(_)) => return Err(mlua::Error::BadArgument { 
                    to: Some("Input:choice".to_string()), 
                    pos: 2, 
                    name: Some("choices".to_string()), 
                    cause: Arc::new(mlua::Error::custom("Argument passed was not the correct type. Must be a query or list.")) 
                }),
            };
            
            Ok(())
        });
        
        methods.add_meta_method_mut("string", |_, this, (name, value) : (String, Option<String>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            this.0.push(InputValue::String { name, value });
            
            Ok(())
        });
        
        methods.add_meta_method_mut("number", |_, this, (name, value) : (String, Option<f64>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            this.0.push(InputValue::Number{ name, value });
            
            Ok(())
        });
        
        methods.add_meta_method_mut("integer", |_, this, (name, value) : (String, Option<i64>)| {
            let Some(this) = Input::from_ref(this) else { return Ok(())};
            let Ok(mut this) = this.write() else { return Ok(()) };
            
            this.0.push(InputValue::Int{ name, value });
            
            Ok(())
        });
    }
}

//==============================================================================================
//        Choice
//==============================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputValue {
    ChoiceQuery {
        name : String,
        query : Query,
        number_of_selections : u32,
        number_of_choices : u32,
        choices : VecDeque<u32>
    },
    Choice {
        name : String,
        values : Vec<StatValue>,
        number_of_selections : u32,
        number_of_choices : u32,
        choices : VecDeque<u32>
    },
    String {
        name : String,
        value : Option<String>
    },
    Number {
        name : String,
        value : Option<f64>
    },
    Int {
        name : String,
        value : Option<i64>
    }
}

impl InputValue {
    pub fn is_choice(&self) -> bool {
        matches!(self, InputValue::Choice { .. } | InputValue::ChoiceQuery { .. })
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, InputValue::String { .. })
    }
    
    pub fn is_number(&self) -> bool {
        matches!(self, InputValue::Number { .. })
    }
    
    pub fn is_int(&self) -> bool {
        matches!(self, InputValue::Int { .. })
    }
    
    pub fn get_name(&self) -> &str {
        match self {
            InputValue::ChoiceQuery { name, .. } => &name,
            InputValue::Choice { name, ..} => &name,
            InputValue::String { name, .. } => &name,
            InputValue::Number { name, .. } => &name,
            InputValue::Int { name, .. } => &name,
        }
    }
    
    pub fn choices_made(&self) -> usize {
        match self {
            InputValue::ChoiceQuery { choices, .. } => choices.len(),
            InputValue::Choice { choices, .. } => choices.len(),
            InputValue::String { value, .. } => if value.is_some() { 1 } else { 0 },
            InputValue::Number { value, .. } => if value.is_some() { 1 } else { 0 },
            InputValue::Int { value, .. } => if value.is_some() { 1 } else { 0 },
        }
    }
    
    pub fn max_choices(&self) -> usize {
        match self {
            InputValue::ChoiceQuery { number_of_choices, .. } => *number_of_choices as usize,
            InputValue::Choice { number_of_choices, .. } => *number_of_choices as usize,
            _ => 1
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.choices_made() >= self.max_choices()
    }
    
    pub fn pop(&mut self) {
        match self {
            InputValue::ChoiceQuery { choices, .. } => { choices.pop_front(); },
            InputValue::Choice { choices, .. } => { choices.pop_front(); },
            InputValue::String { value, .. } => { *value = None },
            InputValue::Number { value, .. } => { *value = None},
            InputValue::Int { value, .. } => { *value = None },
        };
    }
    
    pub fn choose(&mut self, index : usize) {
        match self {
            InputValue::ChoiceQuery { choices, .. } => { choices.push_back(index as u32); },
            InputValue::Choice { choices, .. } => { choices.push_back(index as u32); },
            _ => {}
        };
    }
    
    pub fn set_string(&mut self, string : String) {
        match self {
            Self::String { value, .. } => { *value = Some(string); },
            _ => {}
        }
    }
    
    pub fn set_number(&mut self, number : f64) {
        match self {
            Self::Number { value, .. } => { *value = Some(number); },
            _ => {}
        }
    }
    
    pub fn set_int(&mut self, int : i64) {
        match self {
            Self::Int { value, .. } => { *value = Some(int); },
            _ => {}
        }
    }
}

//==============================================================================================
//        InputValue
//==============================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChoiceValue {
    String(String),
    Number(f64),
    Integer(i64),
}

impl FromLua for ChoiceValue {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let error = Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "ChoiceValue".to_string(), message: None });
        match value {
            Value::String(value) => {Ok(ChoiceValue::String(value.to_string_lossy()))},
            Value::Integer(value) => Ok(ChoiceValue::Integer(value)),
            Value::Number(value) => Ok(ChoiceValue::Number(value)),
            _ => error
        }
    }
}