use std::{fmt::Debug, io::Cursor, path::{Path, PathBuf}};

use base64::{Engine, prelude::BASE64_STANDARD};
use image::ImageFormat;
use mlua::{ExternalResult, FromLua, Lua, UserData, Value};
use pak_db::index::PakSearchable;
use serde::{Deserialize, Serialize};

use crate::{common::{meta::{HasItemMeta, ItemMeta, enable_meta_methods}, util::get_mime_type}, error::{FreError, FreException, FreResult}, lua::LuaSourceMeta};

//==============================================================================================
//        Asset
//==============================================================================================

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    data : String,
    mime : String,
    meta : ItemMeta
}

impl Asset {
    
    pub fn load(name : &str, path : impl AsRef<Path>) -> FreResult<Asset> {
        
        let path = PathBuf::from(path.as_ref());
        let Some(extention) = path.extension() else { return Err(FreError::FileTypeNotValidAsset("NONE".to_string())) };
        
        if extention == "png" || extention == "jpeg" || extention == "jpg" || extention == "webp" {
            let mime = get_mime_type(extention)?;
            let image_format = ImageFormat::from_mime_type(&mime).unwrap();
            
            // Load image and write it to a buffer
            let image = image::open(&path)?;
            let mut buffer = Cursor::new(Vec::new());
            image.write_to(&mut buffer, image_format)?;
            let bytes = buffer.into_inner();
            
            //Convert to base64
            let data = BASE64_STANDARD.encode(bytes);
            
            // Then save the data as a css uri
            return Ok(Asset { 
                meta: ItemMeta::new_extra(name, "Asset", &mime),
                data,
                mime,
            })
        }
        
        Err(FreError::FileTypeNotValidAsset(format!("{extention:?}")))
    }

    pub fn data(&self) -> &str {
        &self.data
    }
    
    pub fn get_asset_type(&self) -> String {
        self.mime.split("/").next().unwrap().to_string()
    }
    
    pub fn is_image(&self) -> bool {
        self.mime.starts_with("image")
    }
}

impl Debug for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Asset").field("mime", &self.mime).field("meta", &self.meta).finish()
    }
}

impl PakSearchable for Asset {
    fn get_indices(&self, indices : &mut pak_db::index::Indices) {
        self.meta.get_indices(indices);
        indices.add("asset_type", self.get_asset_type());
    }
}

impl UserData for Asset {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) { 
        enable_meta_methods(methods);
    }
}

impl HasItemMeta for Asset {
    fn get_meta(&self) -> &ItemMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut ItemMeta {
        &mut self.meta
    }
}

impl FromLua for Asset {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        let Value::UserData(data) = value else {return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Asset".to_string(), message: None }) };
        data.take()
    }
}

//==============================================================================================
//        Assets for Lua
//==============================================================================================

pub fn enable_assets(lua : &Lua) -> FreException {  
    lua.globals().set("asset", lua.create_function(|lua, (id, path) : (String, PathBuf)| {
        let lua_meta = lua.app_data_ref::<LuaSourceMeta>().unwrap();
        let parent = lua_meta.current_file.parent().unwrap();
        let path = parent.join(path);
        println!("{path:?}");
        drop(lua_meta);
        Ok(Asset::load(&id, path).into_lua_err()?)
    })?)?;
    
    Ok(())
}