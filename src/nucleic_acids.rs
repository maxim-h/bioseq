pub mod nucleic_acids {
    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq)]
    struct DNA {
        seq: String,
        quality: Option<String>,
    }

    impl DNA {
        fn new() -> Self {
            DNA { seq: String::from(""), quality: None }
        }

        /// Returns DNA struct with reverse-complementary sequence
        fn rc(&mut self) -> DNA {
            /// Complementarity table for nucleotides
            let ct: HashMap<char, char> =
                [('A', 'T'),
                    ('T', 'A'),
                    ('G', 'C'),
                    ('C', 'G'),
                    ('N', 'N')]
                    .iter().cloned().collect();

            DNA {
                seq: self.seq.chars().map(|x: &char| ct.get(x)).collect(),
                quality: match self.quality {
                    Some(x) => x.chars().rev().collect(),
                    None => None,
                }
            }
        }
    }
}
