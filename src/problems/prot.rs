use itertools::Itertools as _;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::parsing::input;

const STOP_CODON: char = 'Z';

lazy_static! {
    pub static ref CODONS: HashMap<(char, char, char), char> = HashMap::from([
        (('U', 'U', 'U'), 'F'),
        (('C', 'U', 'U'), 'L'),
        (('A', 'U', 'U'), 'I'),
        (('G', 'U', 'U'), 'V'),
        (('U', 'U', 'C'), 'F'),
        (('C', 'U', 'C'), 'L'),
        (('A', 'U', 'C'), 'I'),
        (('G', 'U', 'C'), 'V'),
        (('U', 'U', 'A'), 'L'),
        (('C', 'U', 'A'), 'L'),
        (('A', 'U', 'A'), 'I'),
        (('G', 'U', 'A'), 'V'),
        (('U', 'U', 'G'), 'L'),
        (('C', 'U', 'G'), 'L'),
        (('A', 'U', 'G'), 'M'),
        (('G', 'U', 'G'), 'V'),
        (('U', 'C', 'U'), 'S'),
        (('C', 'C', 'U'), 'P'),
        (('A', 'C', 'U'), 'T'),
        (('G', 'C', 'U'), 'A'),
        (('U', 'C', 'C'), 'S'),
        (('C', 'C', 'C'), 'P'),
        (('A', 'C', 'C'), 'T'),
        (('G', 'C', 'C'), 'A'),
        (('U', 'C', 'A'), 'S'),
        (('C', 'C', 'A'), 'P'),
        (('A', 'C', 'A'), 'T'),
        (('G', 'C', 'A'), 'A'),
        (('U', 'C', 'G'), 'S'),
        (('C', 'C', 'G'), 'P'),
        (('A', 'C', 'G'), 'T'),
        (('G', 'C', 'G'), 'A'),
        (('U', 'A', 'U'), 'Y'),
        (('C', 'A', 'U'), 'H'),
        (('A', 'A', 'U'), 'N'),
        (('G', 'A', 'U'), 'D'),
        (('U', 'A', 'C'), 'Y'),
        (('C', 'A', 'C'), 'H'),
        (('A', 'A', 'C'), 'N'),
        (('G', 'A', 'C'), 'D'),
        (('U', 'A', 'A'), STOP_CODON),
        (('C', 'A', 'A'), 'Q'),
        (('A', 'A', 'A'), 'K'),
        (('G', 'A', 'A'), 'E'),
        (('U', 'A', 'G'), STOP_CODON),
        (('C', 'A', 'G'), 'Q'),
        (('A', 'A', 'G'), 'K'),
        (('G', 'A', 'G'), 'E'),
        (('U', 'G', 'U'), 'C'),
        (('C', 'G', 'U'), 'R'),
        (('A', 'G', 'U'), 'S'),
        (('G', 'G', 'U'), 'G'),
        (('U', 'G', 'C'), 'C'),
        (('C', 'G', 'C'), 'R'),
        (('A', 'G', 'C'), 'S'),
        (('G', 'G', 'C'), 'G'),
        (('U', 'G', 'A'), STOP_CODON),
        (('C', 'G', 'A'), 'R'),
        (('A', 'G', 'A'), 'R'),
        (('G', 'G', 'A'), 'G'),
        (('U', 'G', 'G'), 'W'),
        (('C', 'G', 'G'), 'R'),
        (('A', 'G', 'G'), 'R'),
        (('G', 'G', 'G'), 'G'),
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
