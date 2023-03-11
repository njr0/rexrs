// use std::HashMap;

mod characters;
use characters::*;

/*
mod rex;
use rex::*;
*/

fn main() {
    let path = "/Users/njr/python/genrex/charclasses.json";
    let char_classes = load_char_classes(path).expect("Bad JSON");
    println!("LC_LETTERS rex is {:?}", char_classes);

    /*
    let r = Rex {
        char_classes: HashMap::new(),
    };
    */

}
