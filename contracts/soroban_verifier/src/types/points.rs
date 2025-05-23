use ark_bn254::{Fq, G1Affine};   
use ark_ff::Field;  
use crate::types::Error;  
use crate::utils::BytesReader;  
  
#[derive(Clone, Debug)]  
pub struct G1Point {  
    pub x: Fq,  
    pub y: Fq,  
}  
  
impl G1Point {  
    pub fn to_affine(&self) -> Result<G1Affine, Error> {  
        let point = G1Affine::new_unchecked(self.x, self.y);  
        if !point.is_on_curve() {  
            return Err(Error::InvalidProof);  
        }  
  
        if !point.is_in_correct_subgroup_assuming_on_curve() {  
            return Err(Error::InvalidProof);  
        }  
        Ok(point)  
    }  
}  
  
#[derive(Clone, Debug, Copy)]  
pub struct G1ProofPoint {  
    pub x_0: Fq,  
    pub x_1: Fq,  
    pub y_0: Fq,  
    pub y_1: Fq,  
}  
  
impl G1ProofPoint {  
    pub fn from_reader(reader: &mut BytesReader) -> Self {  
        Self {  
            x_0: reader.read_fq(),  
            x_1: reader.read_fq(),  
            y_0: reader.read_fq(),  
            y_1: reader.read_fq(),  
        }  
    }  
  
    pub fn to_g1_point(&self) -> G1Point {  
        let shift = Fq::from(2u128).pow(&[136u64]);  
        let x = self.x_0 + self.x_1 * shift;  
        let y = self.y_0 + self.y_1 * shift;  
        G1Point { x, y }  
    }  
}