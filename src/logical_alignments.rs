extern crate bit_vec;

use crate::nucleic_acids::{HasLength, DNA};
use bit_vec::*;

pub trait Encode {
    fn encode(&self) -> BitVec;
}

impl Encode for DNA
where
    DNA: HasLength,
{
    fn encode(&self) -> BitVec {
        let bv: BitVec<u32> = bit_vec::BitVec::from_elem((2 * self.len()) as usize, false);
        unimplemented!()
    }
}
