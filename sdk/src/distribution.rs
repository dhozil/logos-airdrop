use std::path::Path;
use anyhow::{Context, Result};
use csv::ReaderBuilder;
use rand::Rng;

use crate::types::{RecipientEntry, AirdropConfig, DistributionManifest};
use crate::merkle::{MerkleTree, build_tree_from_recipients, DEFAULT_TREE_DEPTH};

pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::thread_rng().fill(&mut salt);
    salt
}

pub fn generate_nullifier_secret() -> [u8; 32] {
    let mut secret = [0u8; 32];
    rand::thread_rng().fill(&mut secret);
    secret
}

pub fn parse_recipients_csv(path: &str) -> Result<Vec<RecipientEntry>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(Path::new(path))
        .context("Failed to open CSV file")?;

    let mut recipients = Vec::new();

    for result in rdr.records() {
        let record = result.context("Failed to read CSV record")?;

        let address_hex = record.get(0).context("Missing address column")?;
        let amount_str = record.get(1).context("Missing amount column")?;

        let address = hex_to_bytes32(address_hex)
            .context("Invalid address format")?;
        let amount: u64 = amount_str.parse()
            .context("Invalid amount format")?;

        let salt = generate_salt();
        let nullifier_secret = generate_nullifier_secret();

        recipients.push(RecipientEntry {
            address,
            amount,
            salt,
            nullifier_secret,
        });
    }

    Ok(recipients)
}

pub fn create_distribution(
    recipients: Vec<RecipientEntry>,
    token_program_id: [u8; 32],
    distributor_address: [u8; 32],
    total_allocation: u64,
    tree_depth: usize,
) -> Result<DistributionManifest> {
    let tree = build_tree_from_recipients(&recipients, tree_depth);

    let config = AirdropConfig {
        merkle_root: tree.root,
        token_program_id,
        distributor_address,
        total_allocation,
    };

    Ok(DistributionManifest {
        config,
        recipients,
        tree_depth: DEFAULT_TREE_DEPTH,
    })
}

fn hex_to_bytes32(hex_str: &str) -> Result<[u8; 32]> {
    let hex_str = hex_str.trim();
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);

    let bytes = hex::decode(hex_str)
        .context("Failed to decode hex string")?;

    if bytes.len() != 32 {
        anyhow::bail!("Expected 32 bytes, got {}", bytes.len());
    }

    let mut result = [0u8; 32];
    result.copy_from_slice(&bytes);
    Ok(result)
}

pub struct DistributionCLI;

impl DistributionCLI {
    pub fn generate_manifest(
        csv_path: &str,
        token_program_hex: &str,
        distributor_hex: &str,
        total_allocation: u64,
        output_path: &str,
    ) -> Result<()> {
        let recipients = parse_recipients_csv(csv_path)?;

        let token_program_id = hex_to_bytes32(token_program_hex)?;
        let distributor_address = hex_to_bytes32(distributor_hex)?;

        let manifest = create_distribution(
            recipients,
            token_program_id,
            distributor_address,
            total_allocation,
            DEFAULT_TREE_DEPTH,
        )?;

        let json = serde_json::to_string_pretty(&manifest)?;
        std::fs::write(output_path, &json)
            .context("Failed to write manifest file")?;

        println!("Distribution manifest written to: {output_path}");
        println!("Merkle root: {}", hex::encode(manifest.config.merkle_root));
        println!("Recipients: {}", manifest.recipients.len());

        Ok(())
    }

    pub fn get_proof_for_recipient(
        manifest_path: &str,
        recipient_address_hex: &str,
    ) -> Result<()> {
        let manifest_json = std::fs::read_to_string(manifest_path)?;
        let manifest: DistributionManifest = serde_json::from_str(&manifest_json)?;

        let target = hex_to_bytes32(recipient_address_hex)?;

        let recipient = manifest.recipients.iter()
            .find(|r| r.address == target)
            .context("Recipient not found in distribution")?;

        let leaves: Vec<[u8; 32]> = manifest.recipients.iter()
            .map(|r| crate::merkle::compute_leaf(&r.address, r.amount, &r.salt))
            .collect();

        let index = manifest.recipients.iter()
            .position(|r| r.address == target)
            .unwrap();

        let tree = MerkleTree::new(leaves, manifest.tree_depth);
        let proof = tree.generate_proof(index)
            .context("Failed to generate proof")?;

        let claim = crate::proof::ClaimData {
            nullifier_secret: recipient.nullifier_secret,
            nullifier: crate::proof::compute_nullifier(&recipient.nullifier_secret),
            merkle_path: proof.merkle_path,
            leaf_index: proof.leaf_index,
            recipient_address: recipient.address,
            amount: recipient.amount,
            salt: recipient.salt,
        };

        let claim_json = serde_json::to_string_pretty(&claim)?;
        println!("{claim_json}");

        Ok(())
    }
}
