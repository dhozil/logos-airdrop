use clap::{Parser, Subcommand};
use anyhow::Result;
use airdrop_sdk::distribution::DistributionCLI;

#[derive(Parser)]
#[command(name = "airdrop-cli", about = "Private Airdrop Distributor for LEZ")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a distribution manifest from a CSV of recipients
    Generate {
        /// Path to CSV file with columns: address, amount
        #[arg(short, long)]
        csv: String,

        /// Token program ID (hex)
        #[arg(short, long)]
        token: String,

        /// Distributor address (hex)
        #[arg(short, long)]
        distributor: String,

        /// Total allocation amount
        #[arg(short, long)]
        allocation: u64,

        /// Output path for the manifest JSON
        #[arg(short, long, default_value = "distribution.json")]
        output: String,
    },

    /// Get claim data for a specific recipient
    Proof {
        /// Path to the distribution manifest
        #[arg(short, long, default_value = "distribution.json")]
        manifest: String,

        /// Recipient address (hex)
        #[arg(short, long)]
        address: String,
    },

    /// Display distribution status
    Status {
        /// Path to the distribution manifest
        #[arg(short, long, default_value = "distribution.json")]
        manifest: String,
    },

    /// Serialize an instruction to hex-encoded Risc0-serde Vec<u32>
    Serialize {
        /// Type of instruction: "init", "claim", or "close"
        #[arg(short, long)]
        instruction: String,

        /// Path to claim JSON (for "claim" instruction)
        #[arg(short, long)]
        claim: Option<String>,

        /// Merkle root hex (for "init" instruction)
        #[arg(short = 'r', long)]
        merkle_root: Option<String>,

        /// Distributor hex (for "init" instruction)
        #[arg(short = 'd', long)]
        distributor: Option<String>,

        /// Total allocation (for "init" instruction)
        #[arg(short = 'a', long)]
        allocation: Option<u64>,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { csv, token, distributor, allocation, output } => {
            DistributionCLI::generate_manifest(&csv, &token, &distributor, allocation, &output)?;
        }
        Commands::Proof { manifest, address } => {
            DistributionCLI::get_proof_for_recipient(&manifest, &address)?;
        }
        Commands::Status { manifest } => {
            let json = std::fs::read_to_string(&manifest)?;
            let manifest: airdrop_sdk::types::DistributionManifest = serde_json::from_str(&json)?;
            println!("Distribution Status:");
            println!("  Merkle Root: {}", hex::encode(manifest.config.merkle_root));
            println!("  Total Allocation: {}", manifest.config.total_allocation);
            println!("  Recipients: {}", manifest.recipients.len());
            println!("  Tree Depth: {}", manifest.tree_depth);
        }
        Commands::Serialize { instruction, claim, merkle_root, distributor, allocation } => {
            let instr = match instruction.as_str() {
                "init" => {
                    let root = hex::decode(merkle_root.as_deref().unwrap())?;
                    let dist = hex::decode(distributor.as_deref().unwrap())?;
                    let mut root_arr = [0u8; 32];
                    let mut dist_arr = [0u8; 32];
                    root_arr.copy_from_slice(&root);
                    dist_arr.copy_from_slice(&dist);
                    airdrop_sdk::instructions::Instruction::Initialize {
                        merkle_root: root_arr,
                        distributor: dist_arr,
                        total_allocation: allocation.unwrap(),
                    }
                }
                "claim" => {
                    let claim_json = std::fs::read_to_string(claim.as_deref().unwrap())?;
                    let claim_data: airdrop_sdk::proof::ClaimData = serde_json::from_str(&claim_json)?;
                    airdrop_sdk::instructions::Instruction::Claim {
                        nullifier_secret: claim_data.nullifier_secret,
                        merkle_path: claim_data.merkle_path,
                        leaf_index: claim_data.leaf_index,
                        recipient_address: claim_data.recipient_address,
                        amount: claim_data.amount,
                        salt: claim_data.salt,
                    }
                }
                "close" => airdrop_sdk::instructions::Instruction::Close,
                _ => anyhow::bail!("Unknown instruction: {instruction}. Use: init, claim, close"),
            };
            let serialized = airdrop_sdk::instructions::serialize_instruction(&instr)?;
            let bytes: Vec<u8> = serialized.iter()
                .flat_map(|w| w.to_le_bytes())
                .collect();
            println!("{}", hex::encode(&bytes));
        }
    }

    Ok(())
}
