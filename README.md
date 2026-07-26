# LP-0003: Private Allowlist / Airdrop Distributor

A private token airdrop and allowlist primitive for the Logos Execution Zone (LEZ). Recipients claim allocations without revealing their identity on-chain.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Client SDK (airdrop-cli)                 │
│  - Generate Merkle tree from CSV                            │
│  - Prepare claim proofs                                     │
│  - Submit claims to LEZ                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│              LEZ Airdrop Program (Risc0 Guest)               │
│  initialize(merkle_root, token_id, allocation)              │
│  claim(nullifier_secret, merkle_path, leaf_index,           │
│        recipient_address, amount, salt)                      │
│  close()                                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                     LEZ Sequencer                            │
│  - Verifies ZK proofs                                       │
│  - Updates on-chain state                                   │
│  - Manages nullifier set                                    │
└─────────────────────────────────────────────────────────────┘
```

## Components

| Component | Description | Language |
|-----------|-------------|----------|
| `programs/airdrop/` | LEZ program (Risc0 guest) | Rust (SPEL) |
| `sdk/` | Client-side SDK & CLI | Rust |
| `basecamp/` | Basecamp GUI module | C++/QML |
| `docs/` | Documentation | Markdown |

## Quick Start

```bash
# 1. Build the program
cd programs/airdrop && cargo risczero build --release

# 2. Deploy to LEZ testnet
wallet deploy-program target/risc0-guest/airdrop-program.bin

# 3. Generate distribution manifest
airdrop-cli generate \
  --csv recipients.csv \
  --token <TOKEN_PROGRAM_ID> \
  --distributor <DISTRIBUTOR_ADDRESS> \
  --allocation 100000

# 4. Initialize on-chain
wallet public-tx --program <PROGRAM_ID> \
  --instruction initialize \
  --args <merkle_root> <token_id> <total_allocation>

# 5. Claim (recipient, private execution)
airdrop-cli proof --manifest distribution.json --address <MY_ADDRESS>
wallet private-tx --program <PROGRAM_ID> \
  --instruction claim \
  --args <nullifier_secret> <merkle_path> <leaf_index> <address> <amount> <salt>
```

## Privacy Guarantees

- **Eligibility is hidden**: On-chain state reveals only the Merkle root, not who is eligible
- **Claims are unlinkable**: Each claim produces a fresh nullifier; no on-chain link between claims
- **Amounts are hidden**: Individual allocation amounts are never revealed on-chain
- **Double-claim prevented**: Nullifier scheme prevents multiple claims with the same secret

## Test

```bash
# Run unit tests (dev mode, no real proofs)
RISC0_DEV_MODE=1 cargo test --workspace --release

# Run integration tests
cd sdk && cargo test --release
```

## Demo

See [docs/deployments.md](docs/deployments.md) for live deployments on LEZ testnet.

## License

MIT or Apache-2.0
