use ark_ff::PrimeField;  
use crate::types::{G1Point, G1ProofPoint};  
  
pub fn combine_bytes<F: PrimeField>(a: &F, b: &F) -> F {  
    let shift = F::from(2u128).pow(&[136u64]);  
    *a + *b * shift  
}  
  
pub fn convert_proof_point<F: PrimeField>(input: &G1ProofPoint) -> G1Point {  
    let x = combine_bytes(&input.x_0, &input.x_1);  
    let y = combine_bytes(&input.y_0, &input.y_1);  
    G1Point { x, y }  
}