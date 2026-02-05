pub mod value;
pub mod registry;

use std::{path::{Path, PathBuf}, str::FromStr};

use glob::glob;
use mlua::{AnyUserData, ExternalResult, Lua};
use pak_db::{builder::PakBuilder, Pak};
use serde::{Deserialize, Serialize, ser::Error};
use uuid::Uuid;
use crate::{asset::Asset, common::{deps::RulebookDependency, util::string_or_struct}, error::{BuildError, FreError, FreResult}, lua::{LuaDepsRun, LuaRequireRun, enable_apis, run_file}, rulebook::registry::RulebookRegistry};

const BUILD_SCRIPT_NAME: &str = "familiar.config.json";

pub struct Rulebook {
    pub settings : RulebookMeta,
    pub pak : Pak
}

impl Rulebook {
    
    pub fn from_file(path : impl AsRef<Path>) -> FreResult<Self> {
        let pak = Pak::new_from_file(path)?;
        let settings = pak.get_extra::<RulebookMeta>()?;
        Ok(Rulebook { settings, pak })
    }
    
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
        
        let cover = if let Some(cover) = settings.cover.clone() {
            Some(Asset::load(&format!("{} Cover", settings.name), path.join(cover))?)
        } else {
            None
        };
        
        let extra = RulebookMeta { ruleset : settings.ruleset.clone(), deps : vec![], cover };
        pakker.set_extra(&extra)?;
        
        for (_, item) in objects.into_iter() { pakker.pak(&item)?; }
        for (_, item) in features.into_iter() { pakker.pak(&item)?; }
        for (_, item) in assets.into_iter() { pakker.pak(&item)?; }
        for (_, item) in constructors.into_iter() { pakker.pak(&item)?; }
        
        let pak_path = path.clone().join(format!("{}-{}.rulebook", settings.name.to_lowercase().replace(" ", "-"), settings.version));
        let pak = pakker.build_file(&pak_path)?;
        return Ok(Rulebook { settings : extra, pak });
    }
}

//==============================================================================================
//        Rulebook Settings
//==============================================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RulebookSettings {
    name : String,
    version : String,
    #[serde(deserialize_with = "string_or_struct")]
    ruleset : RulesetInfo,
    cover : Option<String>,
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

    pub fn ruleset(&self) -> &RulesetInfo {
        &self.ruleset
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn cover(&self) -> Option<&String> {
        self.cover.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn authors(&self) -> Option<&Vec<String>> {
        self.authors.as_ref()
    }

    pub fn include(&self) -> Option<&Vec<String>> {
        self.include.as_ref()
    }
}

//==============================================================================================
//        Ruleset Info
//==============================================================================================

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RulesetInfo {
    title : String,
    short : Option<String>
}

impl RulesetInfo {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn short(&self) -> Option<&String> {
        self.short.as_ref()
    }
}

impl FromStr for RulesetInfo {
    type Err = FreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RulesetInfo { title: s.to_string(), ..Default::default()})
    }
}

//==============================================================================================
//        RulebookMeta
//==============================================================================================

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RulebookMeta {
    pub ruleset : RulesetInfo,
    pub deps : Vec<RulebookDependency>,
    pub cover : Option<Asset>
}