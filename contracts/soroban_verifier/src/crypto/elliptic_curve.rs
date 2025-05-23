use ark_ec::{AffineRepr, CurveGroup};  
  
pub fn ec_add<G: CurveGroup>(point0: G::Affine, point1: G::Affine) -> G::Affine {  
    (point0 + point1).into_affine()  
}  
  
pub fn ec_mul<G: CurveGroup>(point: G::Affine, scalar: G::ScalarField) -> G::Affine {  
    (point * scalar).into_affine()  
}  
  
pub fn ec_sub<G: CurveGroup>(point0: G::Affine, point1: G::Affine) -> G::Affine {  
    (point0.into_group() - point1.into_group()).into_affine()  
}