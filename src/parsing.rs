use std::str::FromStr;

pub fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn ints<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    input()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
