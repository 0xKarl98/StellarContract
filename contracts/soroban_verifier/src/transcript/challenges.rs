use ark_bn254::Fr;
use soroban_sdk::Bytes;  
use ark_ff::{PrimeField,Zero};
use ark_serialize::CanonicalSerialize;

use crate::transcript::Transcript;  
use crate::types::{G1Point, Proof};  
use crate::constants::*;  

impl Transcript {  
    pub fn generate_initial_challenge(&mut self, circuit_size: u32, num_inputs: u32) -> Fr {  
        self.append_u32(circuit_size);  
        self.append_u32(num_inputs);  
          
        let challenge_bytes = self.get_challenge();  
        Fr::from_be_bytes_mod_order(&challenge_bytes.to_array())  
    }  
  
    pub fn generate_eta_challenge(&mut self, public_inputs: &[Fr], w1: &G1Point, w2: &G1Point, w3: &G1Point) -> (Fr, Fr, Fr) {  
        for input in public_inputs {  
            let mut bytes = [0u8; 32];  
            input.serialize_compressed(&mut bytes[..]).unwrap();  
            self.append_bytes(&bytes);  
        }  
          
        self.append_g1_point(w1);  
        self.append_g1_point(w2);  
        self.append_g1_point(w3);  
          
        let challenge_bytes = self.get_challenge();  
        let eta = Fr::from_be_bytes_mod_order(&challenge_bytes.to_array());  
        let eta_sqr = eta * eta;  
        let eta_cube = eta_sqr * eta;  
          
        (eta, eta_sqr, eta_cube)  
    }  
  
    pub fn generate_beta_challenge(&mut self, w4: &G1Point, s: &G1Point) -> Fr {  
        self.append_g1_point(w4);  
        self.append_g1_point(s);  
          
        let challenge_bytes = self.get_challenge();  
        Fr::from_be_bytes_mod_order(&challenge_bytes.to_array())  
    }  
  
    pub fn generate_gamma_challenge(&mut self) -> Fr {  
        self.append_bytes(&[0x01]);  
          
        let challenge_bytes = self.get_challenge();  
        Fr::from_be_bytes_mod_order(&challenge_bytes.to_array())  
    }  
  
    pub fn generate_alpha_challenge(&mut self, z: &G1Point, z_lookup: &G1Point) -> (Fr, Fr, Fr, Fr) {  
        self.append_g1_point(z);  
        self.append_g1_point(z_lookup);  
          
        let challenge_bytes = self.get_challenge();  
        let alpha = Fr::from_be_bytes_mod_order(&challenge_bytes.to_array());  
        let alpha_sqr = alpha * alpha;  
        let alpha_cube = alpha_sqr * alpha;  
        let alpha_quad = alpha_cube * alpha;  
          
        (alpha, alpha_sqr, alpha_cube, alpha_quad)  
    }  
  
    pub fn generate_zeta_challenge(&mut self, t1: &G1Point, t2: &G1Point, t3: &G1Point, t4: &G1Point) -> Fr {  
        self.append_g1_point(t1);  
        self.append_g1_point(t2);  
        self.append_g1_point(t3);  
        self.append_g1_point(t4);  
          
        let challenge_bytes = self.get_challenge();  
        Fr::from_be_bytes_mod_order(&challenge_bytes.to_array())  
    }  
  
    pub fn generate_nu_challenges(&mut self, quotient_eval: Fr, proof_data: &[u8]) -> [Fr; 31] {  
        let mut bytes = [0u8; 32];  
        quotient_eval.serialize_compressed(&mut bytes[..]).unwrap();  
        self.append_bytes(&bytes);  
          
        self.append_bytes(proof_data);  
          
        let mut challenges = [Fr::zero(); 31];  
        let base_challenge_bytes = self.get_challenge();  
        let base_challenge = Fr::from_be_bytes_mod_order(&base_challenge_bytes.to_array());  
          
        challenges[0] = base_challenge;  
          
        for i in 1..31 {  
            let mut hasher_input = [0u8; 33];  
            base_challenge.serialize_compressed(&mut hasher_input[..32]).unwrap();  
            hasher_input[32] = i as u8;  
              
            let crypto = self.env.crypto();  
            let hash = crypto.keccak256(&Bytes::from_slice(&self.env, &hasher_input));  
            challenges[i] = Fr::from_be_bytes_mod_order(&hash.to_array());  
        }  
          
        challenges  
    }  
  
    pub fn generate_sumcheck_challenges(&mut self, proof: &Proof) -> [Fr; CONST_PROOF_SIZE_LOG_N] {  
        let mut sumcheck_challenges = [Fr::zero(); CONST_PROOF_SIZE_LOG_N];  
          
        for i in 0..CONST_PROOF_SIZE_LOG_N {  
            let mut univariate_chal = Bytes::new(&self.env);  
              
            let prev_challenge_bytes = self.get_challenge();  
            let prev_challenge = Fr::from_be_bytes_mod_order(&prev_challenge_bytes.to_array());  
              
            let mut challenge_bytes = [0u8; 32];  
            prev_challenge.serialize_compressed(&mut challenge_bytes[..]).unwrap();  
            univariate_chal.extend_from_slice(&challenge_bytes);  
              
            for j in 0..BATCHED_RELATION_PARTIAL_LENGTH {  
                let mut coeff_bytes = [0u8; 32];  
                proof.sumcheck_univariates[i][j].serialize_compressed(&mut coeff_bytes[..]).unwrap();  
                univariate_chal.extend_from_slice(&coeff_bytes);  
            }  
              
            let crypto = self.env.crypto();  
            let hash = crypto.keccak256(&univariate_chal);  
            let challenge = Fr::from_be_bytes_mod_order(&hash.to_array());  
              
            sumcheck_challenges[i] = self.split_challenge(challenge).0;  
              
            self.buffer = Bytes::new(&self.env);  
            self.append_bytes(&hash.to_array());  
        }  
          
        sumcheck_challenges  
    }  
  
    fn split_challenge(&self, challenge: Fr) -> (Fr, Fr) {  
        let challenge_bytes = {  
            let mut bytes = [0u8; 32];  
            challenge.serialize_compressed(&mut bytes[..]).unwrap();  
            bytes  
        };  
          
        let mut lo_bytes = [0u8; 32];  
        let mut hi_bytes = [0u8; 32];  
          
        lo_bytes[16..32].copy_from_slice(&challenge_bytes[16..32]);  
        hi_bytes[16..32].copy_from_slice(&challenge_bytes[0..16]);  
          
        let first = Fr::from_be_bytes_mod_order(&lo_bytes);  
        let second = Fr::from_be_bytes_mod_order(&hi_bytes);  
          
        (first, second)  
    }  
}