mod dna;
mod fib;
mod gc;
mod hamm;
mod iprb;
mod prot;
mod revc;
mod rna;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<String, fn() -> String> = HashMap::from([
        (("dna".to_string(), dna::subject as fn() -> String)),
        (("fib".to_string(), fib::subject as fn() -> String)),
        (("gc".to_string(), gc::subject as fn() -> String)),
        (("hamm".to_string(), hamm::subject as fn() -> String)),
        (("iprb".to_string(), iprb::subject as fn() -> String)),
        (("prot".to_string(), prot::subject as fn() -> String)),
        (("revc".to_string(), revc::subject as fn() -> String)),
        (("rna".to_string(), rna::subject as fn() -> String)),
    ]);
}
