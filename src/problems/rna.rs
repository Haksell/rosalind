use std::io::Read as _;

pub fn subject() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    solve(&input.trim())
}

fn solve(dna: &str) -> String {
    dna.replace("T", "U")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve("GATGGAACTTGACTACGTAAATT"),
            "GAUGGAACUUGACUACGUAAAUU".to_string()
        );
    }
}
