use serde::{Serialize, Deserialize};
use anyhow::Context;

#[derive(Serialize, Deserialize)]
pub enum Instruction {
    Initialize {
        merkle_root: [u8; 32],
        distributor: [u8; 32],
        total_allocation: u64,
    },
    Claim {
        nullifier_secret: [u8; 32],
        merkle_path: Vec<[u8; 32]>,
        leaf_index: u64,
        recipient_address: [u8; 32],
        amount: u64,
        salt: [u8; 32],
    },
    Close,
}

pub fn serialize_instruction(instruction: &Instruction) -> anyhow::Result<Vec<u32>> {
    risc0_zkvm::serde::to_vec(instruction)
        .context("Serialization failed")
}
