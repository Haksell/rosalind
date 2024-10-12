use crate::parsing::{parse_fasta, read_all, Fasta};
use itertools::Itertools as _;

pub fn subject() -> String {
    let fasta = parse_fasta(&read_all());
    let (consensus, counts) = solve(&fasta);
    let counts_fmt = "ACGT"
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let nums = counts.iter().map(|x| x[i].to_string()).join(" ");
            format!("{}: {}", c, nums)
        })
        .join("\n");
    format!("{}\n{}", consensus, counts_fmt)
}

fn solve(dna_strands: &[Fasta]) -> (String, Vec<[usize; 4]>) {
    if dna_strands.is_empty() {
        return (String::new(), Vec::new());
    }
    let len = dna_strands[0].content.len();
    let mut counts = vec![[0; 4]; dna_strands[0].content.len()];
    for dna_strand in dna_strands {
        for (i, c) in dna_strand.content.chars().enumerate() {
            match c {
                'A' => counts[i][0] += 1,
                'C' => counts[i][1] += 1,
                'G' => counts[i][2] += 1,
                'T' => counts[i][3] += 1,
                _ => unreachable!(),
            }
        }
    }
    let consensus = (0..len)
        .map(|i| {
            if counts[i][0] >= counts[i][1]
                && counts[i][0] >= counts[i][2]
                && counts[i][0] >= counts[i][3]
            {
                'A'
            } else if counts[i][1] >= counts[i][2] && counts[i][1] >= counts[i][3] {
                'C'
            } else if counts[i][2] >= counts[i][3] {
                'G'
            } else {
                'T'
            }
        })
        .collect();
    (consensus, counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let (consensus, counts) = solve(&[]);
        assert_eq!(consensus, "");
        assert_eq!(counts, Vec::<[usize; 4]>::new());
    }

    #[test]
    fn test_cons() {
        let input = ">Rosalind_1
ATCCAGCT
>Rosalind_2
GGGCAACT
>Rosalind_3
ATGGATCT
>Rosalind_4
AAGCAACC
>Rosalind_5
TTGGAACT
>Rosalind_6
ATGCCATT
>Rosalind_7
ATGGCACT";
        let fasta = parse_fasta(&input);
        let (consensus, counts) = solve(&fasta);
        assert_eq!(consensus, "ATGCAACT");
        assert_eq!(
            counts,
            vec![
                [5, 0, 1, 1],
                [1, 0, 1, 5],
                [0, 1, 6, 0],
                [0, 4, 3, 0],
                [5, 2, 0, 0],
                [5, 0, 1, 1],
                [0, 6, 0, 1],
                [0, 1, 0, 6],
            ]
        );
    }
}
