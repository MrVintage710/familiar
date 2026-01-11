use std::{collections::{HashMap, HashSet}, path::{Path, PathBuf}};

use line_span::LineSpans;
use lualexer::{FastTokenizer, Lexer, Token, TokenType};
use mlua::{AsChunk, ExternalResult, FromLuaMulti, Function, Lua, Value};
use serde::ser::Error;

use crate::{action::enable_actions, error::VreResult, feature::enable_features, lua::iter::enable_iter, object::enable_objects, stat::{enable_stats, query::enable_query}};

//==============================================================================================
//        LuaSource
//==============================================================================================

#[derive(Debug, Clone)]
pub struct LuaSource {
    path: PathBuf,
    source: String
}

pub struct LuaSourceRef<'a> {
    pub source: &'a LuaSource,
}

impl LuaSource {
    pub fn new<P : AsRef<Path>>(path: P) -> VreResult<Self> {
        let source = std::fs::read_to_string(&path)?;
        let path = PathBuf::from(path.as_ref());
        Ok(LuaSource { path, source})
    }
    
    pub fn run<R : FromLuaMulti>(&self) -> VreResult<R> {
        let lua = mlua::Lua::new();
        self.run_with(&lua)
    }
    
    pub fn run_with<R : FromLuaMulti>(&self, lua : &Lua) -> VreResult<R>  {
        // let path = self.path.to_str().unwrap().to_string();
        // let parent = self.path.parent().unwrap().to_str().unwrap();
        // let package : Table = lua.globals().get("package")?;
        // package.set("path", format!("{parent}/?.lua;;"))?;
        
        let meta = LuaSourceMeta{
            // working_dir : self.path.parent().unwrap().to_path_buf(),
            current_file: self.path.clone(),
            sources : HashMap::from([(self.path.clone(), self.clone())])
        };
        
        lua.set_app_data(meta);
        
        enable_apis(lua)?;
        
        let res = lua.load(&self.source).set_name(self.path.to_str().unwrap().to_string()).eval::<R>()?;
        Ok(res)
    }
    
    pub fn tokenize_function(&self, function : &Function) -> Option<Vec<Token<'_>>> {
        let function_info = function.info();
        let Some(start) = function_info.line_defined else { return None };
        let Some(end) = function_info.last_line_defined else { return None };
        let ranges = self.source.line_spans().skip(start - 1).take((end - start)+1).map(|line| line.range()).collect::<Vec<_>>();
        let range = ranges.first().unwrap().start .. ranges.last().unwrap().end;
        let slice = &self.source[range];
        let tokens = Lexer::<FastTokenizer>::new().parse(slice).unwrap();
        Some(tokens)
    }
    
    pub fn method_dependencies(&self, function : &Function) -> HashSet<String> {
        let mut deps = HashSet::new();
        let Some(tokens) = self.tokenize_function(function) else { return deps };
        let mut iter = tokens.iter();
        while let Some(token) = iter.next() {
            if token.is_type(TokenType::Comment) {
                Self::parse_dep_comment(token.get_content(), &mut deps);
                continue;
            }
            if !(token.is_type(TokenType::Identifier) && token.get_content() == "self") { continue; }
            let mut path = Vec::new();
            loop {
                let Some(punctuation) = iter.next() else { break; };
                if !punctuation.is_type(TokenType::Symbol) { break; };
                if punctuation.get_content() == "." {
                    let Some(identifier) = iter.next() else { break; };
                    if identifier.is_type(TokenType::Identifier) {
                        path.push(identifier.get_content().to_string());
                    }
                } else {
                    break;
                }
            }
            if !path.is_empty() { deps.insert(path.join(".")); }
        };
        deps
    }
    
    fn parse_dep_comment(content : &str, deps : &mut HashSet<String>) {
        if content.starts_with("---@dep") {
            let mut tail = *&content[7..].trim();
            if tail.starts_with("self.") {
                tail = &tail[5..]
            }
            // Simple variable check for now.
            if !tail.contains(" ") {
                deps.insert(tail.to_string());
            }
        }
    }
}

impl AsChunk for LuaSource {
    fn name(&self) -> Option<String> {
        Some(self.path.to_str().unwrap().to_string())
    }

    fn source<'a>(&self) -> std::io::Result<std::borrow::Cow<'a, [u8]>> where Self: 'a {
        self.source.source()
    }
}

pub(crate) fn enable_apis(lua : &Lua, starting_file : impl AsRef<Path>) -> VreResult<()> {
    enable_features(lua)?;
    enable_iter(lua)?;
    enable_objects(lua)?;
    enable_require(lua, starting_file)?;
    enable_actions(lua)?;
    enable_stats(lua)?;
    enable_query(lua)?;
    Ok(())
}

//==============================================================================================
//        LuaSourceMeta
//==============================================================================================

#[derive(Debug)]
pub struct LuaSourceMeta {
    pub current_file : PathBuf,
    pub sources : HashMap<PathBuf, Value>,
}

impl LuaSourceMeta {
    
}

fn enable_require(lua : &Lua, starting_file : impl AsRef<Path>) -> VreResult<()> {
    lua.set_app_data(LuaSourceMeta {
        current_file: PathBuf::from(starting_file.as_ref()),
        sources: HashMap::default(),
    });
    
    lua.globals().set("require", lua.create_function(|lua : &Lua, rel_path : String| {
        let Some(mut meta) = lua.app_data_mut::<LuaSourceMeta>() else { unreachable!()};
        let new_file_path = meta.current_file.parent().unwrap().to_path_buf().join(format!("{rel_path}.lua"));
        
        if new_file_path.exists() || new_file_path.is_dir() { return Err(mlua::Error::custom("The requested file must a valid lua file."))}
        
        if let Some(result) = meta.sources.get(&new_file_path) {
            return Ok(result.clone())
        } else {
            let name = new_file_path.file_name().unwrap().to_string_lossy().to_string();
            let last_path = meta.current_file.clone();
            meta.current_file = new_file_path.clone();
            let source = std::fs::read_to_string(&meta.current_file)?;
            let res = lua.load(source).set_name(name).eval::<Value>()?;
            // let res = source.run_with::<Value>(lua).into_lua_err()?;
            meta.sources.insert(new_file_path.clone(), res.clone());
            meta.current_file = last_path;
            return Ok(res)
        }
        
        // let (name, source, last_path) = {
        //     let last_path = meta.current_file.clone();
        //     meta.current_file = new_file_path.clone();
        //     let source = meta.sources.entry(new_file_path.clone()).or_insert(LuaSource::new(&new_file_path).into_lua_err()?);
        //     let source_code = source.source.clone();
        //     (new_file_path.file_name().unwrap().to_string_lossy().to_string(), source_code, last_path)
        // };
        
        // let res = lua.load(source).set_name(name).eval::<Value>()?;
        
        // if let Some(mut meta) = lua.app_data_mut::<LuaSourceMeta>() {
        //     meta.current_file = last_path;
        // }
        
        // Ok(res)
    })?)?;
    Ok(())
}