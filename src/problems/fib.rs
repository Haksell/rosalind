use crate::parsing::ints;

pub fn subject() -> String {
    let [n, k] = ints::<u128>()[..2] else { todo!() };
    solve(n, k).to_string()
}

fn solve(n: u128, k: u128) -> u128 {
    (1..n).fold((1, 1), |(a, b), _| (b, k * a + b)).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5, 3), 19);
    }
}
