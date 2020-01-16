extern crate ndarray;
use self::ndarray::Array2;
use crate::nucleic_acids::{DNA, Alignment, NucleicAcid, HasLength};
use ndarray::{arr2, ArrayBase, DataOwned, OwnedRepr, Ix2};




pub trait NeedlemanWunschAlign<B> {
    fn global_alignment(&self, other: &B, similarity_matrix: Array2<i8>) -> Alignment<Self, B>
    where Self: Sized + NucleicAcid + HasLength,
    B: NucleicAcid + HasLength;
}

impl<T: NucleicAcid, B: NucleicAcid> NeedlemanWunschAlign<B> for T {
    fn global_alignment(&self, other: &B, similarity_matrix: Array2<i8>) -> Alignment<Self, B>
    where Self: NucleicAcid + Sized + HasLength,
    B: NucleicAcid + HasLength
    {
        let f_matrix: ArrayBase<OwnedRepr<i32>, Ix2> = Array2::zeros((self.len(), other.len()));



        unimplemented!()

    }
}




/*
impl<T, B> Alignment<T, B>
where T: Sized + NucleicAcid,
B: Sized + NucleicAcid
{
    fn global_alignment(&, &other: &T) {

    }
}
*/

