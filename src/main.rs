use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Data {
    pattern: Vec<String>,
    key: HashMap<String, HashMap<String, String>>,
    result: HashMap<String, String>,
}

fn main() {
    let dummy = "planks";

//This is annoying.  Globs in rust are options, and their results are options too, so to extract usable strings, we first
//have to unwrap the glob itself, and then unwrap every element inside

    let results = glob(&format!("recipes/*{}*", dummy.replace(" ", "*")))
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();


    if results.is_empty() {
        println!("Item `{}` not found  ", dummy);
    } else if results.len() > 1 {
        println!(
            "Several items matched your query `{}`: {:?}.  Refine your search and try again.",
            dummy, results
        )
    } else {
	
	//If result only has 1 element, we read the corresponding json 
        let file = fs::read_to_string(&results[0]).expect("unable to read file");
        let crafting_info: Data = serde_json::from_str(&file).expect("unable to parse JSON");
        let needed_items = crafting_info.pattern.join("");
	//Then we count how many of each item we need
        let message = crafting_info
            .key
            .iter()
            .map(|(x, y)| format!("{} {}", needed_items.matches(x).count(), y["item"]))
            .collect::<Vec<_>>();

        println!("{:?}", message)
    }
}
