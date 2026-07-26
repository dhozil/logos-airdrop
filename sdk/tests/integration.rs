use airdrop_sdk::merkle::{MerkleTree, build_tree_from_recipients, compute_leaf};
use airdrop_sdk::proof::compute_nullifier;
use airdrop_sdk::distribution::{generate_salt, generate_nullifier_secret, create_distribution};
use airdrop_sdk::types::RecipientEntry;

#[test]
fn test_full_distribution_flow() {
    let recipients: Vec<RecipientEntry> = (0..10)
        .map(|i| {
            let mut addr = [0u8; 32];
            addr[0] = i;
            RecipientEntry {
                address: addr,
                amount: 1000 * (i as u64 + 1),
                salt: generate_salt(),
                nullifier_secret: generate_nullifier_secret(),
            }
        })
        .collect();

    let token_id = [1u8; 32];
    let distributor = [2u8; 32];
    let total: u64 = recipients.iter().map(|r| r.amount).sum();

    let manifest = create_distribution(
        recipients.clone(),
        token_id,
        distributor,
        total,
        20,
    )
    .expect("Failed to create distribution");

    let tree = build_tree_from_recipients(&recipients, 20);
    assert_eq!(manifest.config.merkle_root, tree.root);

    for (i, recipient) in recipients.iter().enumerate() {
        let leaf = compute_leaf(&recipient.address, recipient.amount, &recipient.salt);
        let proof = tree.generate_proof(i).expect("Failed to generate proof");
        assert!(MerkleTree::verify_proof(&leaf, &proof));

        let nullifier = compute_nullifier(&recipient.nullifier_secret);
        assert_ne!(nullifier, [0u8; 32]);
    }

    let bad_leaf = [99u8; 32];
    let first_proof = tree.generate_proof(0).unwrap();
    assert!(!MerkleTree::verify_proof(&bad_leaf, &first_proof));
}
