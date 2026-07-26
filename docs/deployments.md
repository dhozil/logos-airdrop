# Deployment

## Prerequisites

- Rust toolchain (1.75+)
- Risc0 toolchain: `cargo risczero 2.0`
- LEZ wallet: configured for testnet

## Build

```bash
# Build the airdrop program
cd programs/airdrop
cargo risczero build --release

# Build the SDK/CLI
cd ../../sdk
cargo build --release
```

## Deploy to LEZ Testnet

```bash
# Deploy the compiled program
wallet deploy-program target/risc0-guest/airdrop-program.bin

# Note the deployed program ID
# Program ID: <DEPLOYED_PROGRAM_ID>
```

## Initialize Distribution

```bash
# Fund the distributor account (if needed)
wallet auth-transfer 50000 <DISTRIBUTOR_ADDRESS>

# Initialize the first distribution
wallet public-tx \
  --program <DEPLOYED_PROGRAM_ID> \
  --instruction initialize \
  --args <MERKLE_ROOT> <TOKEN_PROGRAM_ID> 100000 \
  --signer <DISTRIBUTOR_ADDRESS>
```

## Claim Tokens

```bash
# Recipient runs private execution (generates ZK proof)
wallet private-tx \
  --program <DEPLOYED_PROGRAM_ID> \
  --instruction claim \
  --args <NULLIFIER_SECRET> <MERKLE_PATH> <LEAF_INDEX> <RECIPIENT_ADDR> <AMOUNT> <SALT>
```

## Deployed Instances

### Instance A: First Distribution
- **Program ID**: `<to_be_deployed>`
- **Distribution PDA**: `<to_be_computed>`
- **Merkle Root**: `<to_be_filled>`
- **Total Allocation**: 50,000
- **Recipients**: 10
- **Transaction (Initialize)**: `<tx_hash>`

### Instance B: Second Distribution
- **Program ID**: `<to_be_deployed>`
- **Distribution PDA**: `<to_be_computed>`
- **Merkle Root**: `<to_be_filled>`
- **Total Allocation**: 100,000
- **Recipients**: 20
- **Transaction (Initialize)**: `<tx_hash>`
