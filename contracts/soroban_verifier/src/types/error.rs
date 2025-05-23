// Error codes  
#[derive(Clone, Debug)]  
#[repr(u32)]  
pub enum Error {  
    ProofLengthWrong = 0,  
    PublicInputsLengthWrong = 1,  
    SumcheckFailed = 2,  
    ShpleminiFailed = 3,  
    InvalidProof = 4,  
}