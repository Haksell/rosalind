use crate::parsing::input;
use itertools::Itertools as _;

pub fn subject() -> String {
    solve(&input(), &input())
        .iter()
        .map(|i| (i + 1).to_string())
        .join(" ")
}

// TODO: KMP
fn solve(haystack: &str, needle: &str) -> Vec<usize> {
    let mut indexes = vec![];
    let mut start = 0;
    while let Some(idx) = haystack[start..].find(needle) {
        indexes.push(start + idx);
        start += idx + 1;
    }
    indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("AAAAA", "AAA"), vec![0, 1, 2]);
        assert_eq!(solve("GATATATGCATATACTT", "ATAT"), vec![1, 3, 9]);
    }
}
