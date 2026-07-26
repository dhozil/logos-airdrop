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
    }

    Ok(())
}
