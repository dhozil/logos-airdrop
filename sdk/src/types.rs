use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AirdropConfig {
    pub merkle_root: [u8; 32],
    pub token_program_id: [u8; 32],
    pub distributor_address: [u8; 32],
    pub total_allocation: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipientEntry {
    pub address: [u8; 32],
    pub amount: u64,
    pub salt: [u8; 32],
    pub nullifier_secret: [u8; 32],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MerkleProof {
    pub leaf: [u8; 32],
    pub leaf_index: u64,
    pub merkle_path: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DistributionManifest {
    pub config: AirdropConfig,
    pub recipients: Vec<RecipientEntry>,
    pub tree_depth: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClaimOutput {
    pub nullifier: [u8; 32],
    pub recipient_address: [u8; 32],
    pub amount: u64,
    pub transaction_hash: Option<String>,
    pub success: bool,
    pub error: Option<String>,
}
