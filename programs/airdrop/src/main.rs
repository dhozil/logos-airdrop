#![no_main]
#![no_std]

extern crate alloc;

risc0_zkvm::guest::entry!(main);

use alloc::vec::Vec;
use alloc::vec;
use core::convert::TryInto;
use risc0_zkvm::guest::env;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

// ── LEZ protocol types (must match lee_core field order) ──────────────

pub type ProgramId = [u32; 8];
pub type InstructionData = Vec<u32>;

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountId(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct Nonce(pub u128);

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountData(pub Vec<u8>);

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub program_owner: ProgramId,
    pub balance: u128,
    pub data: AccountData,
    pub nonce: Nonce,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountWithMetadata {
    pub account: Account,
    pub is_authorized: bool,
    pub account_id: AccountId,
}

#[derive(Serialize, Deserialize)]
pub enum Claim {
    Authorized,
    Pda(PdaSeed),
}

#[derive(Serialize, Deserialize)]
pub struct AccountPostState {
    pub account: Account,
    pub claim: Option<Claim>,
}

#[derive(Serialize, Deserialize)]
pub struct PdaSeed(pub [u8; 32]);

#[derive(Serialize, Deserialize)]
pub struct ChainedCall {
    pub program_id: ProgramId,
    pub pre_states: Vec<AccountWithMetadata>,
    pub instruction_data: InstructionData,
    pub pda_seeds: Vec<PdaSeed>,
}

#[derive(Serialize, Deserialize)]
pub struct ValidityWindow<T> {
    pub from: Option<T>,
    pub to: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct ProgramOutput {
    pub self_program_id: ProgramId,
    pub caller_program_id: Option<ProgramId>,
    pub instruction_data: InstructionData,
    pub pre_states: Vec<AccountWithMetadata>,
    pub post_states: Vec<AccountPostState>,
    pub chained_calls: Vec<ChainedCall>,
    pub block_validity_window: ValidityWindow<u64>,
    pub timestamp_validity_window: ValidityWindow<u64>,
}

// ── Airdrop types ─────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct DistributionState {
    pub merkle_root: [u8; 32],
    pub distributor: [u8; 32],
    pub total_allocation: u64,
    pub claimed_so_far: u64,
    pub active: bool,
    pub nullifiers: Vec<[u8; 32]>,
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

// ── Manual binary encoding for DistributionState ─────────────────────

fn encode_state(state: &DistributionState) -> Vec<u8> {
    let nullifiers_len = state.nullifiers.len() as u32;
    let mut buf = Vec::with_capacity(32 + 32 + 8 + 8 + 1 + 4 + nullifiers_len as usize * 32);
    buf.extend_from_slice(&state.merkle_root);
    buf.extend_from_slice(&state.distributor);
    buf.extend_from_slice(&state.total_allocation.to_le_bytes());
    buf.extend_from_slice(&state.claimed_so_far.to_le_bytes());
    buf.push(state.active as u8);
    buf.extend_from_slice(&nullifiers_len.to_le_bytes());
    for n in &state.nullifiers {
        buf.extend_from_slice(n);
    }
    buf
}

fn decode_state(data: &[u8]) -> DistributionState {
    let mut offset = 0;
    let mut read_32 = |off: &mut usize| -> [u8; 32] {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&data[*off..*off + 32]);
        *off += 32;
        arr
    };
    let mut read_u64 = |off: &mut usize| -> u64 {
        let bytes: [u8; 8] = data[*off..*off + 8].try_into().unwrap();
        *off += 8;
        u64::from_le_bytes(bytes)
    };

    let merkle_root = read_32(&mut offset);
    let distributor = read_32(&mut offset);
    let total_allocation = read_u64(&mut offset);
    let claimed_so_far = read_u64(&mut offset);
    let active = data[offset] != 0;
    offset += 1;
    let nullifiers_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4;
    let mut nullifiers = Vec::with_capacity(nullifiers_len);
    for _ in 0..nullifiers_len {
        nullifiers.push(read_32(&mut offset));
    }
    DistributionState {
        merkle_root,
        distributor,
        total_allocation,
        claimed_so_far,
        active,
        nullifiers,
    }
}

// ── Merkle proofs ─────────────────────────────────────────────────────

fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let mut output = [0u8; 32];
    output.copy_from_slice(&hasher.finalize());
    output
}

fn compute_leaf(recipient: &[u8; 32], amount: u64, salt: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"airdrop_leaf");
    hasher.update(recipient);
    hasher.update(&amount.to_le_bytes());
    hasher.update(salt);
    let mut leaf = [0u8; 32];
    leaf.copy_from_slice(&hasher.finalize());
    leaf
}

fn compute_nullifier(secret: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"airdrop_nullifier");
    hasher.update(secret);
    let mut nullifier = [0u8; 32];
    nullifier.copy_from_slice(&hasher.finalize());
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

// ── Main ──────────────────────────────────────────────────────────────

fn main() {
    // Read LEZ-standard inputs
    let self_program_id: ProgramId = env::read();
    let caller_program_id: Option<ProgramId> = env::read();
    let pre_states: Vec<AccountWithMetadata> = env::read();
    let instruction_words: InstructionData = env::read();
    let instruction: Instruction =
        risc0_zkvm::serde::from_slice(&instruction_words).expect("Failed to deserialize instruction");

    let pre_states_clone = pre_states.clone();

    let post_states = match instruction {
        Instruction::Initialize { merkle_root, distributor, total_allocation } => {
            let mut account = pre_states
                .into_iter()
                .next()
                .expect("Initialize requires 1 pre-state account")
                .account;

            let state = DistributionState {
                merkle_root,
                distributor,
                total_allocation,
                claimed_so_far: 0,
                active: true,
                nullifiers: Vec::new(),
            };
            let encoded = encode_state(&state);
            account.data = AccountData(encoded);

            vec![AccountPostState {
                account,
                claim: Some(Claim::Authorized),
            }]
        }

        Instruction::Claim {
            nullifier_secret,
            merkle_path,
            leaf_index,
            recipient_address,
            amount,
            salt,
        } => {
            let mut account = pre_states
                .into_iter()
                .next()
                .expect("Claim requires 1 pre-state account")
                .account;

            let state = decode_state(&account.data.0);

            assert!(state.active, "Distribution is not active");
            assert!(
                state.claimed_so_far + amount <= state.total_allocation,
                "Insufficient remaining allocation"
            );

            let nullifier = compute_nullifier(&nullifier_secret);
            assert!(
                !state.nullifiers.contains(&nullifier),
                "Nullifier already claimed"
            );

            let leaf = compute_leaf(&recipient_address, amount, &salt);
            assert!(
                verify_merkle_inclusion(&leaf, &merkle_path, leaf_index, &state.merkle_root),
                "Merkle inclusion proof failed"
            );

            let mut updated = DistributionState {
                claimed_so_far: state.claimed_so_far + amount,
                ..state
            };
            updated.nullifiers.push(nullifier);

            let encoded = encode_state(&updated);
            account.data = AccountData(encoded);

            vec![AccountPostState {
                account,
                claim: None,
            }]
        }

        Instruction::Close => {
            let mut account = pre_states
                .into_iter()
                .next()
                .expect("Close requires 1 pre-state account")
                .account;

            let state = decode_state(&account.data.0);

            let updated = DistributionState {
                active: false,
                ..state
            };
            let encoded = encode_state(&updated);
            account.data = AccountData(encoded);

            vec![AccountPostState {
                account,
                claim: None,
            }]
        }
    };

    let output = ProgramOutput {
        self_program_id,
        caller_program_id,
        instruction_data: instruction_words,
        pre_states: pre_states_clone,
        post_states,
        chained_calls: Vec::new(),
        block_validity_window: ValidityWindow { from: None, to: None },
        timestamp_validity_window: ValidityWindow { from: None, to: None },
    };
    env::write(&output);
}
