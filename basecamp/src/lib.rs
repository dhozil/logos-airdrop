use std::collections::HashMap;
use std::sync::Mutex;
use anyhow::Result;
use airdrop_sdk::types::*;
use airdrop_sdk::merkle::*;
use airdrop_sdk::proof::*;
use airdrop_sdk::distribution::*;

pub struct AirdropBackend {
    distribution: Mutex<Option<DistributionManifest>>,
}

impl AirdropBackend {
    pub fn new() -> Self {
        AirdropBackend {
            distribution: Mutex::new(None),
        }
    }

    pub fn load_manifest(&self, path: &str) -> Result<()> {
        let json = std::fs::read_to_string(path)?;
        let manifest: DistributionManifest = serde_json::from_str(&json)?;
        let mut dist = self.distribution.lock().unwrap();
        *dist = Some(manifest);
        Ok(())
    }

    pub fn check_eligibility(&self, recipient_address: &[u8; 32]) -> Result<Option<RecipientEntry>> {
        let dist = self.distribution.lock().unwrap();
        let dist = dist.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No distribution loaded"))?;

        Ok(dist.recipients.iter()
            .find(|r| r.address == *recipient_address)
            .cloned())
    }

    pub fn prepare_claim_data(
        &self,
        recipient_address: &[u8; 32],
    ) -> Result<Option<ClaimData>> {
        let dist = self.distribution.lock().unwrap();
        let dist = dist.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No distribution loaded"))?;

        let recipient = dist.recipients.iter()
            .find(|r| r.address == *recipient_address);

        match recipient {
            Some(entry) => {
                let leaves: Vec<[u8; 32]> = dist.recipients.iter()
                    .map(|r| airdrop_sdk::merkle::compute_leaf(&r.address, r.amount, &r.salt))
                    .collect();

                let index = dist.recipients.iter()
                    .position(|r| r.address == *recipient_address)
                    .unwrap();

                let tree = MerkleTree::new(leaves, dist.tree_depth);
                let proof = tree.generate_proof(index)?;

                let claim = prepare_claim(
                    &entry.nullifier_secret,
                    &proof,
                    &entry.address,
                    entry.amount,
                    &entry.salt,
                );

                Ok(Some(claim))
            }
            None => Ok(None),
        }
    }
}
