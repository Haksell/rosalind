mod cons;
mod dna;
mod fib;
mod fibd;
mod gc;
mod grph;
mod hamm;
mod iev;
mod ini1;
mod ini2;
mod ini3;
mod ini4;
mod ini5;
mod ini6;
mod iprb;
mod prot;
mod revc;
mod rna;
mod subs;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<String, fn() -> String> = HashMap::from([
        ("cons".to_string(), cons::subject as fn() -> String),
        ("dna".to_string(), dna::subject as fn() -> String),
        ("fib".to_string(), fib::subject as fn() -> String),
        ("fibd".to_string(), fibd::subject as fn() -> String),
        ("gc".to_string(), gc::subject as fn() -> String),
        ("grph".to_string(), grph::subject as fn() -> String),
        ("hamm".to_string(), hamm::subject as fn() -> String),
        ("iev".to_string(), iev::subject as fn() -> String),
        ("ini1".to_string(), ini1::subject as fn() -> String),
        ("ini2".to_string(), ini2::subject as fn() -> String),
        ("ini3".to_string(), ini3::subject as fn() -> String),
        ("ini4".to_string(), ini4::subject as fn() -> String),
        ("ini5".to_string(), ini5::subject as fn() -> String),
        ("ini6".to_string(), ini6::subject as fn() -> String),
        ("iprb".to_string(), iprb::subject as fn() -> String),
        ("prot".to_string(), prot::subject as fn() -> String),
        ("revc".to_string(), revc::subject as fn() -> String),
        ("rna".to_string(), rna::subject as fn() -> String),
        ("subs".to_string(), subs::subject as fn() -> String),
    ]);
}
