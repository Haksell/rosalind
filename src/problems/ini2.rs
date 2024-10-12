pub fn subject() -> String {
    let (a, b) = read!(u32, u32);
    solve(a, b).to_string()
}

fn solve(a: u32, b: u32) -> u32 {
    a * a + b * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(3, 5), 34);
    }
}
