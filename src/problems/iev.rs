pub fn subject() -> String {
    let (a, b, c, d, e, f) = read!(u64, u64, u64, u64, u64, u64);
    solve(a, b, c, d, e, f).to_string()
}

fn solve(a: u64, b: u64, c: u64, d: u64, e: u64, _: u64) -> f64 {
    2.0 * ((a + b + c) as f64 + d as f64 * 0.75 + e as f64 * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_is_close!(solve(1, 0, 0, 1, 0, 1), 3.5);
    }
}
