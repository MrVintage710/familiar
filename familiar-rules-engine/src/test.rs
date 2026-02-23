use std::fs;

use mlua::{FromLuaMulti, Lua};
use pak_db::query::PakQuery;
use crate::{asset::Asset, constructor::Constructor, error::FreResult, feature::Feature, lua::{LuaRequireRun, enable_apis, run_file}, object::Object, rulebook::Rulebook, stat::{field::{StatBlockField, StatSourceProvider}, statblock::StatBlock, value::StatValue}};

pub const EXAMPLE_RULESET : &'static str = "./rulesets/example-ruleset";
pub const TEST_RULESET : &'static str = "./rulesets/test-ruleset";

//==============================================================================================
//        Helper functions
//==============================================================================================

pub fn run_test<R : FromLuaMulti>(file_name : &str) -> FreResult<R> {
    let lua = runtime()?;
    enable_apis(&lua)?;
    let source = format!("{EXAMPLE_RULESET}/{file_name}.lua");
    Ok(R::from_lua_multi(run_file(&lua, source, LuaRequireRun)?, &lua)?)
}

pub fn runtime() -> FreResult<Lua> {
    let lua = Lua::new();
    lua.globals().set("assert", lua.create_function(|_lua : &Lua, value : bool| {
        assert!(value);
        Ok(())
    })?)?;
    Ok(lua)
}

//==============================================================================================
//        Statblock Tests
//==============================================================================================

#[test]
fn statblock_source() {
    let [first_name, last_name] : [StatBlockField; 2] = run_test("statblock_source").unwrap();
    assert_eq!(first_name.get(), StatValue::String("Jane".to_string()));
    assert_eq!(last_name.get(), StatValue::String("Doe".to_string()));
}

#[test]
fn statblock_derive() {
    let [_, _, full_name, _] : [StatBlockField; 4] = run_test("statblock_derive").unwrap();
    assert_eq!(full_name.get(), StatValue::String("John Doe".to_string()))
}

#[test]
fn statblock() {
    let lua = Lua::new();
    let statblock : StatBlock = run_test("statblock").unwrap();
    
    assert_eq!(statblock.get("full_name"), StatValue::String("John Doe".to_string()));
    
    let b = bincode::serialize(&statblock).unwrap();
    let mut stat : StatBlock = bincode::deserialize(&b).unwrap();
    stat.get_field_mut("first_name").unwrap().set(&lua, "Test");
}

#[test]
fn statblock_stress_test() {
    let object : Object = run_test("statblock_stress_test").unwrap();
    
}

//==============================================================================================
//        Feature Tests
//==============================================================================================

#[test]
fn feature_create() {
    let feature : Feature = run_test("feature_create").unwrap();
}

#[test]
fn feature_setup() {
    let lua = Lua::new();
    let feature : Feature = run_test("feature_setup").unwrap();
    
    let mut input = feature.evoke_setup(&lua).unwrap();
}

//==============================================================================================
//        Constructor Tests
//==============================================================================================

#[test]
// fn constructor_create() {
//     let mut constructor : Constructor = run_test("constructor_create").unwrap();
    
//     let mut step_iter = constructor.steps();
//     let (step_name, step) = step_iter.next().unwrap();
//     let mut iterator = step.iter();
//     let (section_name, section) = iterator.next().unwrap();
//     let mut section_iter = section.iter().unwrap();
//     let (choice_name, choice) = section_iter.next().unwrap();
    
//     let json = serde_json::to_string_pretty(&step).unwrap();
//     println!("{json}") 
// }


//==============================================================================================
//        Object Tests
//==============================================================================================

#[test]
fn object_create() {
    let object : Object = run_test("object_create").unwrap();
}

#[test]
fn object_ops() {
    let object : Object = run_test("object_ops").unwrap();
    // assert_eq!(object.assets().len(), 1);
}

//==============================================================================================
//        Rulebook Tests
//==============================================================================================

#[test]
fn rulebook() {
    let rulebook = Rulebook::build(TEST_RULESET).unwrap();
    
    let constructors = rulebook.pak.query::<(Object, Constructor)>(PakQuery::All).unwrap();
    
    println!("{constructors:#?}")
    // fs::remove_file(format!("{EXAMPLE_RULESET}/rulebook.pak")).unwrap();
}

//==============================================================================================
//        Asset Tests
//==============================================================================================

#[test]
fn asset_create() {
    let asset : Asset = run_test("asset_create").unwrap();
    println!("{}", asset.data())
}

//==============================================================================================
//        Chracter Test
//==============================================================================================

#[test]
fn test_character() {
    let character : Object = run_test("test_character").unwrap();
    // std::fs::write("test_character.character", bincode::serialize(&character).unwrap()).unwrap();
}

//==============================================================================================
//        Old Tests
//==============================================================================================


// #[test]
// fn create_object_data() {
//     let source = LuaSource::new("./p2fe-def/character.object.lua").unwrap();
//     let lua = Lua::new();
//     let mut obj : StatBlockData = source.run_with(&lua).unwrap();
    
//     obj.set("attributes.str", 3);
//     obj.set("athletics_proficiency", "trained");
//     obj.resolve(&lua).unwrap();
    
//     assert_eq!(obj.get_value("athletics").unwrap().as_i32(), 6);
// }

// #[test]
// fn object_data_mutation() {
//     let source = LuaSource::new("./p2fe-def/test.object.lua").unwrap();
//     let lua = Lua::new();
//     let mut obj : StatBlockData = source.run_with(&lua).unwrap();
    
//     //Effects working right after creation 
//     assert!(obj.get_value("max_hp").is_some());
//     assert_eq!(obj.get_value("max_hp").unwrap().as_i32(), 12);
    
//     //Effects working after changes from rust
//     obj.set("level", 3);
//     obj.resolve(&lua).unwrap();
    
//     assert!(obj.get_value("max_hp").is_some());
//     assert_eq!(obj.get_value("max_hp").unwrap().as_i32(), 36);
    
//     //Effects working after changes from lua.
//     obj.scope(&lua, |lua, this| {
//         lua.globals().set("obj", this)?;
//         lua.load("obj.level = 10").exec()?;
//         Ok(())
//     }).unwrap();
    
//     assert!(obj.get_value("max_hp").is_some());
//     assert_eq!(obj.get_value("max_hp").unwrap().as_i32(), 120);
    
//     //Checking for no change with read onlymode
//     obj.readonly_scope(&lua, |lua, this| {
//         lua.globals().set("obj", this)?;
//         lua.load("obj.level = 10").exec()?;
//         Ok(())
//     }).unwrap();
    
//     assert!(obj.get_value("max_hp").is_some());
//     assert_eq!(obj.get_value("max_hp").unwrap().as_i32(), 120);
// }

// #[test]
// fn lua_ruleset() {
//     let ruleset = Rulebook::build("./rulesets/example").unwrap();
// }

// #[test]
// fn lua_object() {
//     let source = LuaSource::new("./rulesets/example/monster.lua").unwrap();
//     let object : Object = source.run().unwrap();
    
//     println!("{object:?}")
// }

// #[test]
// fn lua_action() {
//     let source = LuaSource::new("./rulesets/example/default_actions.lua").unwrap();
//     let action : Vec<ActionDef> = source.run().unwrap();
    
//     println!("{action:?}")
// }

// #[test]
// fn lua_iter() {
//     let source = LuaSource::new("./test/iter_test.lua").unwrap();
//     let result : Vec<String> = source.run().unwrap();
//     assert_eq!(result, vec!["str", "int", "wis", "cha"])
// }

// #[test]
// fn lua_feature() {
//     let lua = Lua::new();
//     let feature_source = LuaSource::new("./rulesets/example/level_up.lua").unwrap();
//     let feature : Feature = feature_source.run_with(&lua).unwrap();
    
//     assert_eq!(feature.choices().count(), 1);
//     assert_eq!(feature.choices().nth(0).unwrap().1.choice_count(), 1);
//     assert!(!feature.is_valid())
// }

// #[test]
// fn lua_object_atatch_feature() {
//     let lua = Lua::new();
//     let feature_source = LuaSource::new("");
// }

