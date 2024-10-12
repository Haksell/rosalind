use crate::parsing::input;

pub fn subject() -> String {
    let s = input();
    let (a, b, c, d) = read!(usize, usize, usize, usize);
    solve(&s, a, b, c, d)
}

fn solve(s: &str, a: usize, b: usize, c: usize, d: usize) -> String {
    format!("{} {}", &s[a..=b], &s[c..=d])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let s = "HumptyDumptysatonawallHumptyDumptyhadagreatfallAlltheKingshorsesandalltheKingsmenCouldntputHumptyDumptyinhisplaceagain.";
        assert_eq!(solve(s, 22, 27, 97, 102), "Humpty Dumpty");
    }
}
