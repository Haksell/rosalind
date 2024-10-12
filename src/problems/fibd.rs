pub fn subject() -> String {
    let (n, m) = tints!(2, u128);
    solve(n, m).to_string()
}

fn solve(n: u128, m: u128) -> u128 {
    let mut wabbits = vec![0; m as usize];
    wabbits[0] = 1;
    for _ in 0..n - 1 {
        let new_wabbits = wabbits[1..].iter().sum();
        wabbits.rotate_right(1);
        wabbits[0] = new_wabbits;
    }
    wabbits.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1, 3), 1);
        assert_eq!(solve(2, 3), 1);
        assert_eq!(solve(3, 3), 2);
        assert_eq!(solve(4, 3), 2);
        assert_eq!(solve(5, 3), 3);
        assert_eq!(solve(6, 3), 4);
    }
}
