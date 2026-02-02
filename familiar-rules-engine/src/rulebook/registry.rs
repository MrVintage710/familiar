use std::collections::HashMap;

use mlua::{AnyUserData, ExternalResult, Lua, UserDataRef};
use serde::de::Error;
use uuid::Uuid;

use crate::{asset::Asset, common::{identifier::Identifier, meta::HasItemMeta}, constructor::Constructor, error::{FreError, FreException, FreResult}, feature::Feature, lua::LuaDepsRun, object::{LuaObject, Object}};

//==============================================================================================
//        Build References
//==============================================================================================

#[derive(Default)]
pub struct RulebookRegistry {
    pub objects : HashMap<Uuid, Object>,
    pub features : HashMap<Uuid, Feature>,
    pub assets : HashMap<Uuid, Asset>,
    pub constructors : HashMap<Uuid, Constructor>,
}

impl RulebookRegistry {
    pub fn explode(self) -> (HashMap<Uuid, Object>, HashMap<Uuid, Feature>, HashMap<Uuid, Asset>, HashMap<Uuid, Constructor>) {
        (self.objects, self.features, self.assets, self.constructors)
    }
    
    pub fn register(&mut self, user_data : AnyUserData) -> FreResult<Identifier> {
        if let Ok(object) = user_data.borrow::<LuaObject>() {
            let object = object.read().unwrap();
            let uuid = object.get_meta().uuid();
            if !self.objects.contains_key(&uuid) {
                self.objects.insert(uuid, object.clone());
            }
            return Ok(object.id());
        }
        
        if let Ok(feature) = user_data.borrow::<Feature>() {
            let uuid = feature.get_meta().uuid();
            if !self.features.contains_key(&uuid) {
                self.features.insert(uuid, feature.clone());
            }
            return Ok(feature.id());
        }
        
        if let Ok(asset) = user_data.borrow::<Asset>() {return self.register_asset(&asset)}
        
        if let Ok(constructor) = user_data.borrow::<Constructor>() {
            let uuid = constructor.get_meta().uuid();
            if !self.constructors.contains_key(&uuid) {
                self.constructors.insert(uuid, constructor.clone());
            }
            return Ok(constructor.id());
        }
        
        Err(FreError::CannotRegisterType(user_data.type_name().unwrap().unwrap()))
    }
    
    pub fn register_asset(&mut self, asset : &UserDataRef<Asset>) -> FreResult<Identifier> {
        let uuid = asset.get_meta().uuid();
        if !self.assets.contains_key(&uuid) {
            self.assets.insert(uuid, (*asset).clone());
        }
        return Ok(asset.id());
    }
    
    pub fn register_feature(&mut self, feature : &UserDataRef<Feature>) -> FreResult<Identifier> {
        let uuid = feature.get_meta().uuid();
        if !self.features.contains_key(&uuid) {
            self.features.insert(uuid, (*feature).clone());
        }
        return Ok(feature.id());
    }
}

//==============================================================================================
//        Enable Registry Function
//==============================================================================================

pub fn enable_regisrty(lua : &Lua) -> FreException {
    lua.set_app_data(RulebookRegistry::default());
    
    lua.globals().set("register", lua.create_function(|lua : &Lua, user_data : AnyUserData| {
        let mut registry = lua.app_data_mut::<RulebookRegistry>().ok_or(mlua::Error::custom("Rulebook Registry not intialized. Report this bug to devs."))?;
        let result = registry.register(user_data).into_lua_err()?;
        if let Some(mut deps) = lua.app_data_mut::<LuaDepsRun>() {
            deps.0.push(result.clone());
        }
        Ok(result)
    })?)?;
    
    Ok(())
}