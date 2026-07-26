#![no_main]
#![no_std]

extern crate alloc;

risc0_zkvm::guest::entry!(main);

use alloc::vec::Vec;
use risc0_zkvm::guest::env;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize)]
pub struct DistributionState {
    pub merkle_root: [u8; 32],
    pub distributor: [u8; 32],
    pub total_allocation: u64,
    pub claimed_so_far: u64,
    pub active: bool,
}

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

#[derive(Serialize, Deserialize)]
pub struct ClaimOutput {
    pub nullifier: [u8; 32],
    pub recipient_address: [u8; 32],
    pub amount: u64,
}

fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

fn compute_leaf(recipient: &[u8; 32], amount: u64, salt: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"airdrop_leaf");
    hasher.update(recipient);
    hasher.update(&amount.to_le_bytes());
    hasher.update(salt);
    let result = hasher.finalize();
    let mut leaf = [0u8; 32];
    leaf.copy_from_slice(&result);
    leaf
}

fn compute_nullifier(secret: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"airdrop_nullifier");
    hasher.update(secret);
    let result = hasher.finalize();
    let mut nullifier = [0u8; 32];
    nullifier.copy_from_slice(&result);
    nullifier
}

fn verify_merkle_inclusion(
    leaf: &[u8; 32],
    merkle_path: &[[u8; 32]],
    leaf_index: u64,
    merkle_root: &[u8; 32],
) -> bool {
    let mut current = *leaf;
    let mut index = leaf_index;

    for sibling in merkle_path {
        if index % 2 == 0 {
            current = hash_pair(&current, sibling);
        } else {
            current = hash_pair(sibling, &current);
        }
        index /= 2;
    }

    current == *merkle_root
}

fn main() {
    let instruction: Instruction = env::read();

    match instruction {
        Instruction::Initialize { merkle_root, distributor, total_allocation } => {
            let state = DistributionState {
                merkle_root,
                distributor,
                total_allocation,
                claimed_so_far: 0,
                active: true,
            };
            env::write(&state);
        }

        Instruction::Claim {
            nullifier_secret,
            merkle_path,
            leaf_index,
            recipient_address,
            amount,
            salt,
        } => {
            let state: DistributionState = env::read();

            assert!(state.active, "Distribution is not active");
            assert!(
                state.claimed_so_far + amount <= state.total_allocation,
                "Insufficient remaining allocation"
            );

            let leaf = compute_leaf(&recipient_address, amount, &salt);
            let inclusion_valid = verify_merkle_inclusion(
                &leaf,
                &merkle_path,
                leaf_index,
                &state.merkle_root,
            );
            assert!(inclusion_valid, "Merkle inclusion proof failed");

            let nullifier = compute_nullifier(&nullifier_secret);

            let output = ClaimOutput {
                nullifier,
                recipient_address,
                amount,
            };

            let updated = DistributionState {
                claimed_so_far: state.claimed_so_far + amount,
                ..state
            };

            env::write(&updated);
            env::write(&output);
        }

        Instruction::Close => {
            let mut state: DistributionState = env::read();
            state.active = false;
            env::write(&state);
        }
    }
}
