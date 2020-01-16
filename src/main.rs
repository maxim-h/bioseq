extern crate ndarray;

use ndarray::{arr2, Array2};
mod dynamic_alignments;
mod logical_alignments;
mod nucleic_acids;

use dynamic_alignments::*;
use logical_alignments::*;
use nucleic_acids::*;

fn main() {
    let s = DNA {
        seq: String::from("ACATCGTACGTGATC"),
        quality: None,
    };

    println!("{:?}", s.seq);

    println!("{:?}", s.rc().sequence().chars().rev().collect::<String>());

    let scores: Array2<i8>  = arr2(&[
        [10, -1, -3, -4],
        [-1, 7, -5, -3],
        [-3, -5, 9, 0],
        [-4, -3, 0, 8],
    ]);


    println!("{:?}", scores);
}
