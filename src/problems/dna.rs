use crate::parsing::input;
use itertools::Itertools as _;

pub fn subject() -> String {
    solve(&input()).iter().map(usize::to_string).join(" ")
}

fn solve(input: &str) -> [usize; 4] {
    let mut counts = [0; 4];
    for c in input.chars() {
        match c {
            'A' => counts[0] += 1,
            'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
            _ => unreachable!(),
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC"),
            [20, 12, 17, 21]
        );
    }
}
