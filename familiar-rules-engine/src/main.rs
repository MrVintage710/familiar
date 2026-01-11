use std::path::PathBuf;
use clap::{command, value_parser, Arg, ArgMatches, Command};

//==============================================================================================
//        Command Line Functions
//==============================================================================================

pub fn main() {
    
    let object_create_command = Command::new("create")
        .about("Creates an object and stores it in a local file.")
        .arg(Arg::new("path").required(true).value_parser(value_parser!(PathBuf)));
    
    let matches = command!()
        .subcommand(
            Command::new("object")
                .about("This subcommand will give you access to everything having to do with objects.")
                .subcommand(object_create_command)
        )
        .subcommand(
            Command::new("build")
                .arg(Arg::new("path").required(true).value_parser(value_parser!(PathBuf)))
                .about("Builds a rules directory into a binary file.")
        )
        .get_matches();
    
    // if let Some(matches) = matches.subcommand_matches("object") {
    //     if let Some(matches) = matches.subcommand_matches("create") {
    //         let path = matches.get_one::<PathBuf>("path");
            
    //         let object = Object::from_file(path.unwrap()).expect("Error");
            
    //         println!("Creating from Path: {path:?} {object:?}")
    //     }
    // }
    
    if let Some(matches) = matches.subcommand_matches("build") { build(matches); }
}

fn build(args : &ArgMatches) -> Option<()> {
    let path : &PathBuf = args.get_one("path")?;
    // Rulebook::build(path).unwrap();
    
    return Some(());
}