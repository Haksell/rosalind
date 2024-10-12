pub fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn read_all() -> String {
    std::io::read_to_string(std::io::stdin()).unwrap()
}

// pub fn read_vec<T>() -> Vec<T>
// where
//     T: FromStr,
//     T::Err: std::fmt::Debug,
// {
//     input()
//         .split_whitespace()
//         .map(|s| s.parse::<T>().unwrap())
//         .collect()
// }

macro_rules! read {
    ( $( $t:ty ),* ) => {
        {
            let stdin = std::io::stdin();
            let mut line = String::new();
            stdin.read_line(&mut line)
                .expect("Failed to read line");
            let mut iter = line.trim().split_whitespace();
            (
                $(
                    iter.next()
                        .expect("Not enough input values")
                        .parse::<$t>()
                        .expect("Failed to parse input"),
                )*
            )
        }
    };
}

#[derive(Debug)]
pub struct Fasta {
    pub name: String,
    pub content: String,
}

impl Fasta {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}

pub fn parse_fasta(input: &str) -> Vec<Fasta> {
    let mut dna_strands = vec![];
    for part in input.split('>').skip(1) {
        let mut parts = part.splitn(2, '\n');
        let name = parts.next().unwrap().to_string();
        let content = parts.next().unwrap().replace("\n", "");
        dna_strands.push(Fasta::new(name, content));
    }
    dna_strands
}

pub fn read_fasta() -> Vec<Fasta> {
    parse_fasta(&read_all())
}
