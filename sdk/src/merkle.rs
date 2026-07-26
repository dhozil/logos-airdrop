use sha2::{Sha256, Digest};
use crate::types::{RecipientEntry, MerkleProof};

pub const DEFAULT_TREE_DEPTH: usize = 20;

fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let combined = [left.as_slice(), right.as_slice()].concat();
    hasher.update(&combined);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

pub fn compute_leaf(recipient: &[u8; 32], amount: u64, salt: &[u8; 32]) -> [u8; 32] {
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

pub struct MerkleTree {
    pub depth: usize,
    pub leaves: Vec<[u8; 32]>,
    pub layers: Vec<Vec<[u8; 32]>>,
    pub root: [u8; 32],
}

impl MerkleTree {
    pub fn new(leaves: Vec<[u8; 32]>, depth: usize) -> Self {
        let filled = Self::fill_leaves(leaves, depth);

        let mut layers = vec![filled.clone()];
        let mut current = filled.clone();

        while current.len() > 1 {
            let mut next = Vec::with_capacity((current.len() + 1) / 2);
            for chunk in current.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() > 1 { chunk[1] } else { [0u8; 32] };
                next.push(hash_pair(&left, &right));
            }
            layers.push(next.clone());
            current = next;
        }

        let root = current[0];

        MerkleTree {
            depth,
            leaves: filled,
            layers,
            root,
        }
    }

    fn fill_leaves(leaves: Vec<[u8; 32]>, depth: usize) -> Vec<[u8; 32]> {
        let target_len = 1 << depth;
        let mut filled = leaves;
        while filled.len() < target_len {
            filled.push([0u8; 32]);
        }
        filled
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Option<MerkleProof> {
        if leaf_index >= self.leaves.len() {
            return None;
        }

        let leaf = self.leaves[leaf_index];
        let mut path = Vec::new();
        let mut idx = leaf_index;

        for layer in &self.layers {
            if layer.len() == 1 {
                break;
            }
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            if sibling_idx < layer.len() {
                path.push(layer[sibling_idx]);
            } else {
                path.push([0u8; 32]);
            }
            idx /= 2;
        }

        Some(MerkleProof {
            leaf,
            leaf_index: leaf_index as u64,
            merkle_path: path,
            root: self.root,
        })
    }

    pub fn verify_proof(leaf: &[u8; 32], proof: &MerkleProof) -> bool {
        let mut current = *leaf;
        let mut idx = proof.leaf_index;

        for sibling in &proof.merkle_path {
            current = if idx % 2 == 0 {
                hash_pair(&current, sibling)
            } else {
                hash_pair(sibling, &current)
            };
            idx /= 2;
        }

        current == proof.root
    }
}

pub fn build_tree_from_recipients(
    recipients: &[RecipientEntry],
    depth: usize,
) -> MerkleTree {
    let leaves: Vec<[u8; 32]> = recipients
        .iter()
        .map(|r| compute_leaf(&r.address, r.amount, &r.salt))
        .collect();

    MerkleTree::new(leaves, depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_proof_verification() {
        let leaves = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];

        let tree = MerkleTree::new(leaves.clone(), 4);

        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.generate_proof(i).unwrap();
            assert!(MerkleTree::verify_proof(leaf, &proof));
        }
    }

    #[test]
    fn test_invalid_proof_rejected() {
        let leaves = vec![[1u8; 32], [2u8; 32]];
        let tree = MerkleTree::new(leaves, 4);
        let proof = tree.generate_proof(0).unwrap();
        let wrong_leaf = [99u8; 32];
        assert!(!MerkleTree::verify_proof(&wrong_leaf, &proof));
    }
}
