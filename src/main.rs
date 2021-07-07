// use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

mod filepath;

fn main() {
    // println!("Hello, world!");
    let cwd = filepath::cwd().expect("Incorrect Permissions");
    println!("The cwd = {}", cwd.display());

    let json_file = filepath::adjecent_file("dice_sides.json", None, None);
    
    let mut data = String::new();
    let mut f = File::open(json_file).expect("Unable to read file");
    f.read_to_string(&mut data).expect("Unable to read string");
    // println!("{}", data);
    let json: Jfile = serde_json::from_str(&data).expect("JSON was not well formatted");

    // println!("Dice color {}")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Sides {
}

// impl Results {
//     fn sides(&self) {

//     }
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Dice {
    name: String,
    color: String,
    sides: i8,
    results: Sides
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Jfile {
    Boost: Dice
}

enum Die {
    Boost,
    Setback,
    Ability,
    Difficulty,
    Proficency,
    Force
}