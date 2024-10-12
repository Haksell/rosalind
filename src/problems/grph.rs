use crate::parsing::{read_fasta, Fasta};
use itertools::Itertools as _;

pub fn subject() -> String {
    let fasta = read_fasta();
    solve(&fasta, 3)
        .iter()
        .map(|(a, b)| format!("{a} {b}"))
        .join("\n")
}

fn solve(strands: &[Fasta], k: usize) -> Vec<(&String, &String)> {
    strands
        .iter()
        .permutations(2)
        .filter(|pair| {
            let a = pair[0];
            let b = pair[1];
            let na = a.content.len();
            let nb = b.content.len();
            na >= k && nb >= k && a.content[na - k..] == b.content[..k]
        })
        .map(|pair| (&pair[0].name, &pair[1].name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_fasta;

    #[test]
    fn test_solve() {
        let fasta = parse_fasta(
            ">Rosalind_0498
AAATAAA
>Rosalind_2391
AAATTTT
>Rosalind_2323
TTTTCCC
>Rosalind_0442
AAATCCC
>Rosalind_5013
GGGTGGG",
        );
        assert_eq!(
            solve(&fasta, 3),
            vec![
                (&"Rosalind_0498".to_string(), &"Rosalind_2391".to_string()),
                (&"Rosalind_0498".to_string(), &"Rosalind_0442".to_string()),
                (&"Rosalind_2391".to_string(), &"Rosalind_2323".to_string()),
            ]
        );
    }
}
