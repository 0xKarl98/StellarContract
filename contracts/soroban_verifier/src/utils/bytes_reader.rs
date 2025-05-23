use ark_bn254::{Fq, Fr};  
use ark_ff::PrimeField;  
  
pub struct BytesReader<'a> {  
    bytes: &'a [u8],  
    pos: usize,  
}  
  
impl<'a> BytesReader<'a> {  
    pub fn new(bytes: &'a [u8]) -> Self {  
        Self { bytes, pos: 0 }  
    }  
      
    pub fn read_u32(&mut self) -> u32 {  
        let mut arr = [0u8; 4];  
        arr.copy_from_slice(&self.bytes[self.pos..self.pos + 4]);  
        self.pos += 4;  
        u32::from_be_bytes(arr)  
    }  
      
    pub fn read_fq(&mut self) -> Fq {  
        let mut arr = [0u8; 32];  
        arr.copy_from_slice(&self.bytes[self.pos..self.pos + 32]);  
        self.pos += 32;  
        Fq::from_be_bytes_mod_order(&arr)  
    }  
      
    pub fn read_fr(&mut self) -> Fr {  
        let mut arr = [0u8; 32];  
        arr.copy_from_slice(&self.bytes[self.pos..self.pos + 32]);  
        self.pos += 32;  
        Fr::from_be_bytes_mod_order(&arr)  
    }  
}