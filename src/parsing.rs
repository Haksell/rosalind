use std::io::Read as _;

pub fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s.trim().to_string()
}
