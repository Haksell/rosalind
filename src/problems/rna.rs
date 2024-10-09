use crate::parsing::input;

pub fn subject() -> String {
    solve(&input())
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
