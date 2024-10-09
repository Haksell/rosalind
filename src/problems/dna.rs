use itertools::Itertools as _;
use std::io::Read as _;

pub fn subject() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    solve(&input.trim()).iter().map(usize::to_string).join(" ")
}

fn solve(input: &str) -> [usize; 4] {
    let mut counts = [0; 4];
    for c in input.chars() {
        match c {
            'A' => counts[0] += 1,
            'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
            _ => unreachable!(),
        }
    }
    counts
}
