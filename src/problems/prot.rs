use itertools::Itertools as _;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::parsing::input;

const STOP_CODON: char = 'Z';

lazy_static! {
    pub static ref CODONS: HashMap<(char, char, char), char> = HashMap::from([
        (('A', 'A', 'A'), 'K'),
        (('A', 'A', 'C'), 'N'),
        (('A', 'A', 'G'), 'K'),
        (('A', 'A', 'U'), 'N'),
        (('A', 'C', 'A'), 'T'),
        (('A', 'C', 'C'), 'T'),
        (('A', 'C', 'G'), 'T'),
        (('A', 'C', 'U'), 'T'),
        (('A', 'G', 'A'), 'R'),
        (('A', 'G', 'C'), 'S'),
        (('A', 'G', 'G'), 'R'),
        (('A', 'G', 'U'), 'S'),
        (('A', 'U', 'A'), 'I'),
        (('A', 'U', 'C'), 'I'),
        (('A', 'U', 'G'), 'M'),
        (('A', 'U', 'U'), 'I'),
        (('C', 'A', 'A'), 'Q'),
        (('C', 'A', 'C'), 'H'),
        (('C', 'A', 'G'), 'Q'),
        (('C', 'A', 'U'), 'H'),
        (('C', 'C', 'A'), 'P'),
        (('C', 'C', 'C'), 'P'),
        (('C', 'C', 'G'), 'P'),
        (('C', 'C', 'U'), 'P'),
        (('C', 'G', 'A'), 'R'),
        (('C', 'G', 'C'), 'R'),
        (('C', 'G', 'G'), 'R'),
        (('C', 'G', 'U'), 'R'),
        (('C', 'U', 'A'), 'L'),
        (('C', 'U', 'C'), 'L'),
        (('C', 'U', 'G'), 'L'),
        (('C', 'U', 'U'), 'L'),
        (('G', 'A', 'A'), 'E'),
        (('G', 'A', 'C'), 'D'),
        (('G', 'A', 'G'), 'E'),
        (('G', 'A', 'U'), 'D'),
        (('G', 'C', 'A'), 'A'),
        (('G', 'C', 'C'), 'A'),
        (('G', 'C', 'G'), 'A'),
        (('G', 'C', 'U'), 'A'),
        (('G', 'G', 'A'), 'G'),
        (('G', 'G', 'C'), 'G'),
        (('G', 'G', 'G'), 'G'),
        (('G', 'G', 'U'), 'G'),
        (('G', 'U', 'A'), 'V'),
        (('G', 'U', 'C'), 'V'),
        (('G', 'U', 'G'), 'V'),
        (('G', 'U', 'U'), 'V'),
        (('U', 'A', 'A'), STOP_CODON),
        (('U', 'A', 'C'), 'Y'),
        (('U', 'A', 'G'), STOP_CODON),
        (('U', 'A', 'U'), 'Y'),
        (('U', 'C', 'A'), 'S'),
        (('U', 'C', 'C'), 'S'),
        (('U', 'C', 'G'), 'S'),
        (('U', 'C', 'U'), 'S'),
        (('U', 'G', 'A'), STOP_CODON),
        (('U', 'G', 'C'), 'C'),
        (('U', 'G', 'G'), 'W'),
        (('U', 'G', 'U'), 'C'),
        (('U', 'U', 'A'), 'L'),
        (('U', 'U', 'C'), 'F'),
        (('U', 'U', 'G'), 'L'),
        (('U', 'U', 'U'), 'F'),
    ]);
}

pub fn subject() -> String {
    solve(&input())
}

fn solve(rna: &str) -> String {
    rna.chars()
        .tuples()
        .map(|triplet| CODONS.get(&triplet).unwrap())
        .take_while(|&&c| c != STOP_CODON)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA"),
            "MAMAPRTEINSTRING"
        );
    }
}
