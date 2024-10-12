use std::{io::Read as _, str::FromStr};

pub fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn read_all() -> String {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
}

pub fn vints<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    input()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}

macro_rules! tints {
    (2, $t:ty) => {{
        let v = crate::parsing::vints::<$t>();
        if v.len() != 2 {
            panic!("Expected 2 elements, but got {}", v.len());
        }
        (v[0], v[1])
    }};
    (3, $t:ty) => {{
        let v = crate::parsing::vints::<$t>();
        if v.len() != 3 {
            panic!("Expected 3 elements, but got {}", v.len());
        }
        (v[0], v[1], v[2])
    }};
}

pub struct Fasta {
    pub name: String,
    pub content: String,
}

impl Fasta {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}

pub fn parse_fasta(input: &str) -> Vec<Fasta> {
    let mut dna_strands = vec![];
    for part in input.split('>').skip(1) {
        let mut parts = part.splitn(2, '\n');
        let name = parts.next().unwrap().to_string();
        let content = parts.next().unwrap().replace("\n", "");
        dna_strands.push(Fasta::new(name, content));
    }
    dna_strands
}
