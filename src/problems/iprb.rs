pub fn subject() -> String {
    let (hod, het, hor) = tints!(3, u64);
    solve(hod, het, hor).to_string()
}

fn solve(hod: u64, het: u64, hor: u64) -> f64 {
    let (hod, het, hor) = (hod as f64, het as f64, hor as f64);
    let total_pairs = pairs(hod + het + hor);
    let dominant_children = pairs(hod) + hod * (het + hor) + pairs(het) * 0.75 + (het * hor) * 0.5;
    dominant_children / total_pairs
}

fn pairs(x: f64) -> f64 {
    x * (x - 1.0) * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_is_close!(solve(2, 2, 2), 0.783333);
    }
}
