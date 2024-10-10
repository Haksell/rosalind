use crate::parsing::read_all;

pub fn subject() -> String {
    let mut dna_strands = vec![];
    for part in read_all().split('>').skip(1) {
        let mut parts = part.splitn(2, '\n');
        let name = parts.next().unwrap().to_string();
        let content = parts.next().unwrap().replace("\n", "");
        dna_strands.push((name, content));
    }
    let (name, gc_content) = solve(&dna_strands);
    format!("{}\n{}", name, gc_content * 100.0)
}

fn solve(dna_strands: &[(String, String)]) -> (&String, f32) {
    dna_strands
        .iter()
        .map(|(name, content)| (name, calculate_gc_content(content)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
}

fn calculate_gc_content(dna: &str) -> f32 {
    dna.chars().filter(|&c| c == 'C' || c == 'G').count() as f32 / dna.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = [
            (
                "Rosalind_6404".to_string(),
                [
                    "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC",
                    "TCCCACTAATAATTCTGAGG",
                ]
                .join(""),
            ),
            (
                "Rosalind_5959".to_string(),
                [
                    "CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT",
                    "ATATCCATTTGTCAGCAGACACGC",
                ]
                .join(""),
            ),
            (
                "Rosalind_0808".to_string(),
                [
                    "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC",
                    "TGGGAACCTGCGGGCAGTAGGTGGAAT",
                ]
                .join(""),
            ),
        ];
        let (name, gc_content) = solve(&input);
        assert_eq!(name, "Rosalind_0808");
        assert_is_close!(gc_content, 0.6091954);
    }
}
