use soroban_sdk::{Env, Bytes, BytesN, U256};  
use ark_serialize::CanonicalSerialize;  
use crate::types::G1Point;  
  
pub struct Transcript {  
    pub env: Env,  
    pub buffer: Bytes,  
}  
  
impl Transcript {  
    pub fn new(env: &Env) -> Self {  
        Self {  
            env: env.clone(),  
            buffer: Bytes::new(env),  
        }  
    }  
  
    pub fn append_bytes(&mut self, bytes: &[u8]) {  
        self.buffer.extend_from_slice(bytes);  
    }  
  
    pub fn append_u32(&mut self, value: u32) {  
        let bytes = value.to_be_bytes();  
        self.append_bytes(&bytes);  
    }  
  
    pub fn append_u256(&mut self, value: &U256) {  
        let bytes: Bytes = value.to_be_bytes();  
        let mut arr = [0u8; 32];  
        bytes.copy_into_slice(&mut arr);  
        self.buffer.extend_from_slice(&arr);  
    }  
  
    pub fn append_g1_point(&mut self, point: &G1Point) {  
        let mut x_bytes = [0u8; 32];  
        let mut y_bytes = [0u8; 32];  
        point.x.serialize_compressed(&mut x_bytes[..]).unwrap();  
        point.y.serialize_compressed(&mut y_bytes[..]).unwrap();  
          
        self.append_bytes(&y_bytes); // Note: Solidity adds y coordinate first  
        self.append_bytes(&x_bytes); // Then x coordinate  
    }  
  
    pub fn get_challenge(&mut self) -> BytesN<32> {  
        let crypto = self.env.crypto();  
        let hash = crypto.keccak256(&self.buffer);  
        self.buffer = Bytes::new(&self.env);  
        hash.to_bytes()  
    }  
  
}