pub mod validator_set;
pub mod voting;
pub mod finality;
pub mod attestation;

#[cfg(test)]
mod attestation_tests;

pub use validator_set::{ValidatorSet, ValidatorInfo};
pub use attestation::{ValidatorAttestation, ValidatorSignature};
