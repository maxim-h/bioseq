use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq)]
pub struct DNA {
    pub seq: String,
    pub quality: Option<String>,
}

impl DNA {
    pub fn new() -> Self {
        DNA {
            seq: String::new(),
            quality: None,
        }
    }
}
// Perhaps could be implemented through Sized trait
pub trait HasLength {
    fn len(&self) -> usize;
}

impl HasLength for DNA {
    fn len(&self) -> usize {
        self.seq.len() as usize
    }
}

pub trait NucleicAcid {
    fn sequence(&self) -> &String;

    fn quality(&self) -> &Option<String>;
}

impl NucleicAcid for DNA {
    fn sequence(&self) -> &String {
        &self.seq
    }

    fn quality(&self) -> &Option<String> {
        &self.quality
    }
}

pub trait FromRaw {
    fn from_raw(s: String, q: Option<String>) -> Self;
}

impl FromRaw for DNA {
    fn from_raw(s: String, q: Option<String>) -> Self {
        DNA { seq: s, quality: q }
    }
}

pub trait Complementary {
    fn rc(&self) -> Self
    where
        Self: NucleicAcid;
}

impl<T> Complementary for T
where
    T: NucleicAcid + FromRaw,
{
    /// Returns DNA struct with reverse-complementary sequence
    fn rc(&self) -> Self {
        let ct: HashMap<char, char> = [('A', 'T'), ('T', 'A'), ('G', 'C'), ('C', 'G'), ('N', 'N')]
            .iter()
            .cloned()
            .collect();

        Self::from_raw(
            String::from_iter(
                self.sequence()
                    .chars()
                    .rev()
                    .map(|x: char| ct.get(&x).unwrap()),
            ),
            match &self.quality() {
                Some(x) => Some(String::from_iter(x.chars().rev())),
                None => None,
            },
        )
    }
}


pub struct Alignment<'a, T, B>
where T: NucleicAcid,
B: NucleicAcid
{
    s1: &'a str,
    s2: &'a str,
    score: u32,
    nucleic_acids: (T, B)
}

impl<'a, T: NucleicAcid, B: NucleicAcid> Alignment<'a, T, B> {}