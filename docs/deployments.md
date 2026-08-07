# Deployment

## Prerequisites

- Rust toolchain (1.75+)
- Risc0 toolchain: `cargo risczero 2.0` (guest built with `risc0-zkvm 3.0.5`)
- LEZ wallet CLI v0.2.2, configured for testnet

## Build

```bash
# Build the airdrop program (guest ELF)
cd programs/airdrop
cargo risczero build --release

# Build the SDK/CLI
cd ../../sdk
cargo build --release --bin airdrop-cli
```

## Deploy to LEZ Testnet

```bash
# Deploy the compiled program
wallet deploy-program target/risc0-guest/airdrop-program.bin

# Note the deployed program ID
```

## Initialize Distribution

```bash
# Initialize the admin (distributor) account first
wallet auth-transfer init --account-id Public/<ADMIN>

# Initialize the distribution (both admin and state account must sign)
wallet call \
  --program <PROGRAM_ID> \
  --data <INIT_HEX> \
  --accounts "Public/<ADMIN>" "Public/<STATE_ACCOUNT>"
```

## Claim Tokens

```bash
# Each claim is a public transaction (admin + state sign)
wallet call \
  --program <PROGRAM_ID> \
  --data <CLAIM_HEX> \
  --accounts "Public/<ADMIN>" "Public/<STATE_ACCOUNT>"
```

## Deployed Instances (LEZ Testnet, chain reset 2026-08)

### Program
- **Program ID**: `26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721`
- **Deployment TX**: `7e2947e72a7720bd53eb40dd5691832a4264d8cc46c653aedd96cb7ca4e4d881`
- **Admin (distributor)**: `AKy1PsJFCR7LBQMdCjH8G3GYmNGcc3gs293bPGEboKSs`

### Instance A: First Distribution
- **State Account**: `BxajpycZ2zbodcxLT6jLkgnnbiqeA4VifNfGF2RDT6X5`
- **Merkle Root**: `8baa5eabdeb8a1e2375d7efe55945e701e5e96d4818fc03f06bb7838bf282b5f`
- **Total Allocation**: 100,000
- **Recipients**: 22
- **Transaction (Initialize)**: `a373b0512011ea9a10fe8243cb6fc96c2645fb324ab70426dd19898de87149c0` (block 959)
- **Claims executed**: 22 (claimed 47,300)

### Instance B: Second Distribution
- **State Account**: `89555exkSkc1zuAZFzt2DmJn46gjFxSiw9DDveV5dAsm`
- **Merkle Root**: `0d0fa522e3bb193b56bdc0b0a978ef757cf2f090b9304620e8f6b4e7fa6d505f`
- **Total Allocation**: 100,000
- **Recipients**: 21
- **Transaction (Initialize)**: `2c0cee1d034fa9e6561a59634275837784f81722d9b03faea86bb3c9ecc87686` (block 996)
- **Claims executed**: 21 (claimed 21,000)

> **Note**: 2 distributions with a combined 43 claims are live on LEZ testnet.
