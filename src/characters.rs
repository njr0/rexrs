use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result};


#[derive(Serialize, Deserialize, Debug)]
pub enum Kind {
    CharSet,
    CharUnion
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CharClass {
    pub kind: Kind,
    pub code: char,
    pub rex: String,
    pub size: u32,
}



pub fn load_char_classes(path: &str) -> Result<HashMap<String, CharClass>> {
    let data = fs::read_to_string(path)
                   .expect("Should have been able to read the file");
    let v = serde_json::from_str(&data)?;
    Ok(v)
}


// TESTS

#[cfg(test)]
mod tests {
    #[test]
    fn chars_it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


