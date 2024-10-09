use crate::parsing::input;

pub fn subject() -> String {
    solve(&input())
}

fn solve(dna: &str) -> String {
    dna.chars()
        .map(|c| match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => unreachable!(),
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("AAAACCCGGT"), "ACCGGGTTTT");
    }
}
