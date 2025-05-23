use ark_bn254::{Bn254, G1Affine, G2Affine, Fq12};  
use ark_ec::pairing::Pairing;  
use ark_ff::One;  
  
pub fn pairing_check(  
    rhs: G1Affine,  
    lhs: G1Affine,  
    fixed_g2: G2Affine,  
    vk_g2: G2Affine,  
) -> bool {  
    let pairing1 = Bn254::pairing(&rhs, &fixed_g2);  
    let pairing2 = Bn254::pairing(&lhs, &vk_g2);  
  
    let prod = pairing1.0 * pairing2.0;  
  
    prod == Fq12::one()  
}