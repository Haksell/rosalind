use std::io::Read as _;

pub fn subject() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    solve(&input.trim())
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
