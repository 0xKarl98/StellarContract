use ark_bn254::Fr;  
use ark_ff::Zero;  
use crate::constants::*;  
use crate::types::{Error, G1ProofPoint};  
use crate::utils::BytesReader;  
  
#[derive(Clone, Debug)]  
pub struct Proof {  
    pub circuit_size: u32,  
    pub public_inputs_size: u32,  
    pub public_inputs_offset: u32,  
    pub w1: G1ProofPoint,  
    pub w2: G1ProofPoint,  
    pub w3: G1ProofPoint,  
    pub w4: G1ProofPoint,  
    pub z_perm: G1ProofPoint,  
    pub lookup_read_counts: G1ProofPoint,  
    pub lookup_read_tags: G1ProofPoint,  
    pub lookup_inverses: G1ProofPoint,  
    pub sumcheck_univariates: [[Fr; BATCHED_RELATION_PARTIAL_LENGTH]; CONST_PROOF_SIZE_LOG_N],  
    pub sumcheck_evaluations: [Fr; NUMBER_OF_ENTITIES],  
    pub gemini_fold_comms: [G1ProofPoint; CONST_PROOF_SIZE_LOG_N - 1],  
    pub gemini_a_evaluations: [Fr; CONST_PROOF_SIZE_LOG_N],  
    pub shplonk_q: G1ProofPoint,  
    pub kzg_quotient: G1ProofPoint,  
}  
  
impl Proof {  
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {  
        let mut reader = BytesReader::new(bytes);  
  
        let circuit_size = reader.read_u32();  
        let public_inputs_size = reader.read_u32();  
        let public_inputs_offset = reader.read_u32();  
  
        let w1 = G1ProofPoint::from_reader(&mut reader);  
        let w2 = G1ProofPoint::from_reader(&mut reader);  
        let w3 = G1ProofPoint::from_reader(&mut reader);  
        let lookup_read_counts = G1ProofPoint::from_reader(&mut reader);  
        let lookup_read_tags = G1ProofPoint::from_reader(&mut reader);  
        let w4 = G1ProofPoint::from_reader(&mut reader);  
        let lookup_inverses = G1ProofPoint::from_reader(&mut reader);  
        let z_perm = G1ProofPoint::from_reader(&mut reader);  
  
        // sumcheck_univariates  
        let mut sumcheck_univariates =  
            [[Fr::zero(); BATCHED_RELATION_PARTIAL_LENGTH]; CONST_PROOF_SIZE_LOG_N];  
        for i in 0..CONST_PROOF_SIZE_LOG_N {  
            for j in 0..BATCHED_RELATION_PARTIAL_LENGTH {  
                sumcheck_univariates[i][j] = reader.read_fr();  
            }  
        }  
  
        // sumcheck_evaluations  
        let mut sumcheck_evaluations = [Fr::zero(); NUMBER_OF_ENTITIES];  
        for i in 0..NUMBER_OF_ENTITIES {  
            sumcheck_evaluations[i] = reader.read_fr();  
        }  
  
        // gemini_fold_comms  
        let mut gemini_fold_comms = [G1ProofPoint {  
            x_0: ark_bn254::Fq::zero(),  
            x_1: ark_bn254::Fq::zero(),  
            y_0: ark_bn254::Fq::zero(),  
            y_1: ark_bn254::Fq::zero(),  
        }; CONST_PROOF_SIZE_LOG_N - 1];  
        for i in 0..(CONST_PROOF_SIZE_LOG_N - 1) {  
            gemini_fold_comms[i] = G1ProofPoint::from_reader(&mut reader);  
        }  
  
        // gemini_a_evaluations  
        let mut gemini_a_evaluations = [Fr::zero(); CONST_PROOF_SIZE_LOG_N];  
        for i in 0..CONST_PROOF_SIZE_LOG_N {  
            gemini_a_evaluations[i] = reader.read_fr();  
        }  
  
        let shplonk_q = G1ProofPoint::from_reader(&mut reader);  
        let kzg_quotient = G1ProofPoint::from_reader(&mut reader);  
  
        Ok(Proof {  
            circuit_size,  
            public_inputs_size,  
            public_inputs_offset,  
            w1,  
            w2,  
            w3,  
            w4,  
            z_perm,  
            lookup_read_counts,  
            lookup_read_tags,  
            lookup_inverses,  
            sumcheck_univariates,  
            sumcheck_evaluations,  
            gemini_fold_comms,  
            gemini_a_evaluations,  
            shplonk_q,  
            kzg_quotient,  
        })  
    }  
}