use std::str::FromStr;

pub fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
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

macro_rules! mints {
    (2, $t:ty) => {{
        let v = crate::parsing::vints::<$t>();
        if v.len() != 2 {
            panic!("Expected 2 elements, but got {}", v.len());
        }
        (v[0], v[1])
    }};
}
