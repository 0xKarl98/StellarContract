//Note this is just draft implementation of honkVerifier logic over stellar soroban  
//We haven't implemented BN254 precompile for now , so use arkwork's bn254 instead  
#![no_std]  
extern crate alloc;  
use wee_alloc::WeeAlloc;  
  
//We keep this allocator as WASM need a memory allocator to handle dynamic memory allocation.  
#[global_allocator]  
static ALLOC: WeeAlloc = WeeAlloc::INIT;  
  
pub mod constants;  
pub mod types;  
pub mod utils;  
pub mod transcript;  
pub mod crypto;  
  
pub use types::{Error, G1Point, G1ProofPoint, Proof, VerificationKey};  
pub use transcript::Transcript;  
pub use crypto::{ec_add, ec_mul, ec_sub, pairing_check};