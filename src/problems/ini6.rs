use crate::parsing::input;
use itertools::Itertools as _;
use std::collections::HashMap;

pub fn subject() -> String {
    solve(&input())
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .join("\n")
}

fn solve(input: &str) -> HashMap<&str, usize> {
    input
        .split_whitespace()
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word).or_insert(0) += 1;
            counts
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve("We tried list and we tried dicts also we tried Zen"),
            HashMap::from([
                ("tried".into(), 3),
                ("we".into(), 2),
                ("and".into(), 1),
                ("We".into(), 1),
                ("dicts".into(), 1),
                ("list".into(), 1),
                ("also".into(), 1),
                ("Zen".into(), 1),
            ])
        );
    }
}

/*
and 1
We 1
tried 3
dicts 1
list 1
we 2
also 1
Zen 1
*/
