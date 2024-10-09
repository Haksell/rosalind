mod dna;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<String, fn() -> String> = {
        let mut map = HashMap::new();
        map.insert("dna".to_string(), dna::subject as fn() -> String);
        map
    };
}
