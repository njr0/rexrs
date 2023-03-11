// use counter::Counter;
use std::collections::HashMap;

mod characters;
use characters::*;


pub struct Rex {
    char_classes: HashMap<String,CharClass>,
}




// TESTS

#[cfg(test)]
mod tests {
    #[test]
    fn rex_it_works() {
        let r = Rex {
            char_classes: HashMap::new(),
        };
        assert_eq!(4, 4);
    }
}
