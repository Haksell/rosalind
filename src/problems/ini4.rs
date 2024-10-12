pub fn subject() -> String {
    let (a, b) = read!(u64, u64);
    solve(a, b).to_string()
}

fn solve(a: u64, b: u64) -> u64 {
    ((b + 1) >> 1).pow(2) - (a >> 1).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1, 2), 1);
        assert_eq!(solve(1, 3), 4);
        assert_eq!(solve(1, 4), 4);
        assert_eq!(solve(1, 5), 9);
        assert_eq!(solve(2, 3), 3);
        assert_eq!(solve(2, 4), 3);
        assert_eq!(solve(2, 5), 8);
        assert_eq!(solve(3, 4), 3);
        assert_eq!(solve(3, 5), 8);
        assert_eq!(solve(4, 5), 5);
        assert_eq!(solve(100, 199), 7500);
        assert_eq!(solve(100, 200), 7500);
        assert_eq!(solve(101, 199), 7500);
        assert_eq!(solve(101, 200), 7500);
    }
}
