use sha2::{Sha256, Digest};
use serde::Serialize;
use crate::types::MerkleProof;

pub fn compute_nullifier(secret: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"airdrop_nullifier");
    hasher.update(secret);
    let result = hasher.finalize();
    let mut nullifier = [0u8; 32];
    nullifier.copy_from_slice(&result);
    nullifier
}

#[derive(Serialize)]
pub struct ClaimData {
    pub nullifier_secret: [u8; 32],
    pub nullifier: [u8; 32],
    pub merkle_path: Vec<[u8; 32]>,
    pub leaf_index: u64,
    pub recipient_address: [u8; 32],
    pub amount: u64,
    pub salt: [u8; 32],
}

pub fn prepare_claim(
    nullifier_secret: &[u8; 32],
    merkle_proof: &MerkleProof,
    recipient_address: &[u8; 32],
    amount: u64,
    salt: &[u8; 32],
) -> ClaimData {
    let nullifier = compute_nullifier(nullifier_secret);

    ClaimData {
        nullifier_secret: *nullifier_secret,
        nullifier,
        merkle_path: merkle_proof.merkle_path.clone(),
        leaf_index: merkle_proof.leaf_index,
        recipient_address: *recipient_address,
        amount,
        salt: *salt,
    }
}

pub fn generate_claim_proof(claim: &ClaimData) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(claim)?)
}
