use crate::parsing::read_all;
use itertools::Itertools as _;

pub fn subject() -> String {
    solve(&read_all()).to_string()
}

fn solve(input: &str) -> String {
    input.lines().skip(1).step_by(2).join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(
                "Bravely bold Sir Robin rode forth from Camelot
Yes, brave Sir Robin turned about
He was not afraid to die, O brave Sir Robin
And gallantly he chickened out
He was not at all afraid to be killed in nasty ways
Bravely talking to his feet
Brave, brave, brave, brave Sir Robin
He beat a very brave retreat"
            ),
            "Yes, brave Sir Robin turned about
And gallantly he chickened out
Bravely talking to his feet
He beat a very brave retreat"
        );
    }
}
