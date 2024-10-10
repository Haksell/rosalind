use crate::parsing::input;

pub fn subject() -> String {
    solve(&input(), &input()).to_string()
}

fn solve(s1: &str, s2: &str) -> usize {
    assert_eq!(s1.len(), s2.len());
    s1.chars().zip(s2.chars()).filter(|&(a, b)| a != b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), 7);
    }
}
