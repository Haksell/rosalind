use crate::parsing::{parse_fasta, read_all, Fasta};

pub fn subject() -> String {
    let fasta = parse_fasta(&read_all());
    let (name, gc_content) = solve(&fasta);
    format!("{}\n{}", name, gc_content * 100.0)
}

fn solve(dna_strands: &[Fasta]) -> (&String, f64) {
    dna_strands
        .iter()
        .map(|fasta| (&fasta.name, calculate_gc_content(&fasta.content)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
}

fn calculate_gc_content(dna: &str) -> f64 {
    dna.chars().filter(|&c| c == 'C' || c == 'G').count() as f64 / dna.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";
        let fasta = parse_fasta(&input);
        let (name, gc_content) = solve(&fasta);
        assert_eq!(name, "Rosalind_0808");
        assert_is_close!(gc_content, 0.6091954);
    }
}
