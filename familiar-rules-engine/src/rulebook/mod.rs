pub mod value;

use std::{collections::HashMap, path::{Path, PathBuf}};

use glob::glob;
use mlua::{AnyUserData, ExternalResult, Lua};
use pak_db::{builder::PakBuilder, Pak};
use serde::{Deserialize, Serialize, ser::Error};
use uuid::Uuid;
use crate::{asset::Asset, common::{deps::RulebookDependency, identifier::Identifier, meta::HasItemMeta}, constructor::Constructor, error::{BuildError, FreError, FreResult}, feature::Feature, lua::{LuaDepsRun, LuaRequireRun, LuaSourceMeta, enable_apis, run_file}, object::Object};

const BUILD_SCRIPT_NAME: &str = "familiar.config.json";

pub struct Rulebook {
    settings : RulebookSettings,
    pub file : Pak
}

impl Rulebook {
    /// This function runs the build file at the given folder path.
    pub fn build(path : impl AsRef<Path>) -> FreResult<Self> {
        //Make path buf
        let path = PathBuf::from(path.as_ref());
        
        //Get values from rulebook.familiar.json
        let mut settings = RulebookSettings::from_path(&path)?;
        
        //Setup the lua environment
        let lua = Lua::new();
        enable_apis(&lua)?;
        lua.set_app_data(settings.clone());
        lua.set_app_data(RulebookRegistry::default());
        
        lua.globals().set("register", lua.create_function(|lua : &Lua, user_data : AnyUserData| {
            let mut registry = lua.app_data_mut::<RulebookRegistry>().ok_or(mlua::Error::custom("Rulebook Registry not intialized. Report this bug to devs."))?;
            let result = registry.register(user_data).into_lua_err()?;
            if let Some(mut deps) = lua.app_data_mut::<LuaDepsRun>() {
                deps.0.push(result.clone());
            }
            Ok(result)
        })?)?;
        
        let includes = settings.include.take().unwrap_or(vec!["**/*.lua".to_string()]);
        let paths : Vec<PathBuf> = includes
            .iter()
            .filter_map(|pattern| glob(path.join(pattern).to_str().unwrap()).ok())
            .map(|entry| entry.into_iter().filter_map(|glob| glob.ok()).collect::<Vec<_>>())
            .flatten()
            .collect()
        ;
        
        for path in paths.iter() { run_file(&lua, &path, LuaRequireRun)?; }
        
        let (objects, features, assets, constructors) = lua.remove_app_data::<RulebookRegistry>().expect("Error: Registry is null.").explode();
        let mut pakker = PakBuilder::new()
            .with_name(&settings.name)
            .with_description(settings.description.as_ref().unwrap_or(&"".to_string()))
            .with_author(settings.authors.as_ref().unwrap_or(&vec![]).join(", ").as_str())
        ;
        
        pakker.set_extra(&RulebookMeta { game_system : settings.ttrpg.clone(), deps : vec![] })?;
        
        for (_, item) in objects.into_iter() { pakker.pak(&item)?; }
        for (_, item) in features.into_iter() { pakker.pak(&item)?; }
        for (_, item) in assets.into_iter() { pakker.pak(&item)?; }
        for (_, item) in constructors.into_iter() { pakker.pak(&item)?; }
        
        let pak_path = path.clone().join(format!("{}-{}.rulebook", settings.name.to_lowercase().replace(" ", "-"), settings.version));
        let pak = pakker.build_file(&pak_path)?;
        return Ok(Rulebook { settings, file: pak });
        
        // let build_src = LuaSource::new(&build_file_path)?;
        
        
        // Set default globals to set
        // lua.globals().set("name", "rulebook")?;
        // lua.globals().set("version", "1.0.0")?;
        // lua.globals().set("game_system", "unknown")?;
        // lua.globals().set("deps", Vec::<String>::new())?;
        
        
        
        //Add the pak builder so that the file will be built
        
        // lua.globals().set("register_feature", lua.create_function(|lua : &Lua, feature : FeatureDef| {
        //     if let Some(mut pak_builder) = lua.app_data_mut::<PakBuilder>() {
        //         pak_builder.pak(feature).expect("Error while building feature.");
        //     }
        //     return Ok(Value::Nil);
        // })?)?;
        
        //Run the build script
        // run_file_with_lua::<Value>(&lua, build_file_path)?;
        
        //Meta Variables
        // let name : String = lua.globals().get("name").unwrap();
        // let version : String = lua.globals().get("version").unwrap();
        // let game_system : String = lua.globals().get("game_system").unwrap();
        // let deps : Vec<RulebookDependency> = lua.globals().get("deps").unwrap();
        
        // //Remove the pak builder after use
        // let mut pak_builder = lua.remove_app_data::<PakBuilder>().unwrap();
        
        // let pak_path = path.clone().join(format!("{}-{version}.rulebook", name.to_lowercase().replace(" ", "-")));
        // pak_builder.set_version(version);
        // pak_builder.set_name(&name);
        // pak_builder.set_extra(&RulebookMeta { game_system, deps })?;
        // let pak = pak_builder.build_file(pak_path)?;
        // return Ok(Rulebook { name, file: pak });
    }
}

//==============================================================================================
//        Rulebook Settings
//==============================================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RulebookSettings {
    name : String,
    version : String,
    ttrpg : String,
    description : Option<String>,
    authors : Option<Vec<String>>,
    include : Option<Vec<String>>
}

impl RulebookSettings {
    
    pub fn from_file(path : impl AsRef<Path>) -> FreResult<Self> {
        let source = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&source)?)
    }
    
    /// This function will find the project settings file based on the given path.
    /// It will check the directory given, then works its way up until it finds
    /// the settings file. Then it loads it into memory.
    pub fn from_path(path : impl AsRef<Path>) -> FreResult<Self> {
        let path = PathBuf::from(path.as_ref());
        if path.is_file() {
            if path.file_name().unwrap() == BUILD_SCRIPT_NAME {
                return Self::from_file(&path);
            } else {
                return Self::from_path(path.clone().parent().ok_or(BuildError::BuildFileNotFound(path))?);
            }
        }
        
        let build_file_path = path.join(BUILD_SCRIPT_NAME);
        if !build_file_path.exists() { return Self::from_path(path.clone().parent().ok_or(BuildError::BuildFileNotFound(path))?)}
        
        Self::from_file(build_file_path)
    }
    
    pub fn uuid(&self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_X500, format!("{}|{}", self.name, self.version).as_bytes())
    }
}

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
    fn explode(self) -> (HashMap<Uuid, Object>, HashMap<Uuid, Feature>, HashMap<Uuid, Asset>, HashMap<Uuid, Constructor>) {
        (self.objects, self.features, self.assets, self.constructors)
    }
    
    fn register(&mut self, user_data : AnyUserData) -> FreResult<Identifier> {
        if let Ok(object) = user_data.borrow::<Object>() {
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
        
        if let Ok(asset) = user_data.borrow::<Asset>() {
            let uuid = asset.get_meta().uuid();
            if !self.assets.contains_key(&uuid) {
                self.assets.insert(uuid, asset.clone());
            }
            return Ok(asset.id());
        }
        
        if let Ok(constructor) = user_data.borrow::<Constructor>() {
            let uuid = constructor.get_meta().uuid();
            if !self.constructors.contains_key(&uuid) {
                self.constructors.insert(uuid, constructor.clone());
            }
            return Ok(constructor.id());
        }
        
        Err(FreError::CannotRegisterType(user_data.type_name().unwrap().unwrap()))
    }
}

//==============================================================================================
//        RulebookMeta
//==============================================================================================

#[derive(Serialize, Deserialize, Default)]
pub struct RulebookMeta {
    pub game_system : String,
    pub deps : Vec<RulebookDependency>
}