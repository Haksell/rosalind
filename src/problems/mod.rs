mod dna;
mod fib;
mod gc;
mod hamm;
mod iprb;
mod revc;
mod rna;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<String, fn() -> String> = {
        let mut map = HashMap::new();
        map.insert("dna".to_string(), dna::subject as fn() -> String);
        map.insert("fib".to_string(), fib::subject as fn() -> String);
        map.insert("gc".to_string(), gc::subject as fn() -> String);
        map.insert("hamm".to_string(), hamm::subject as fn() -> String);
        map.insert("iprb".to_string(), iprb::subject as fn() -> String);
        map.insert("revc".to_string(), revc::subject as fn() -> String);
        map.insert("rna".to_string(), rna::subject as fn() -> String);
        map
    };
}
