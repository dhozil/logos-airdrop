# LP-0003: Private Allowlist / Airdrop Distributor

A private token airdrop and allowlist primitive for the Logos Execution Zone (LEZ). Recipients claim allocations without revealing their identity on-chain.

## Live on LEZ Testnet

- **Program ID**: `26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721`
- **2 distributions, 43 claims confirmed** (Distribution A: 22 claims; Distribution B: 21 claims)
- Deployment TX: `7e2947e72a7720bd53eb40dd5691832a4264d8cc46c653aedd96cb7ca4e4d881`
- Full details: [docs/deployments.md](docs/deployments.md)

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
│  initialize(merkle_root, distributor, allocation)           │
│  claim(nullifier_secret, merkle_path, leaf_index,           │
│        recipient_address, amount, salt)                      │
│  close()                                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                     LEZ Sequencer                            │
│  - Re-executes public transactions (Risc0 zkVM)             │
│  - Validates Merkle inclusion & nullifiers                  │
│  - Updates on-chain state                                   │
└─────────────────────────────────────────────────────────────┘
```

## Components

| Component | Description | Language |
|-----------|-------------|----------|
| `programs/airdrop/` | LEZ program (Risc0 guest) | Rust (Risc0 zkVM 3.0.5) |
| `sdk/` | Client-side SDK & CLI (`airdrop-cli`) | Rust |
| `basecamp/` | Basecamp GUI module | C++/QML |
| `docs/` | Documentation (deployments, CU costs) | Markdown |

## Quick Start

### 1. Build the program

```bash
cd programs/airdrop
cargo risczero build --release
```

### 2. Deploy to LEZ testnet

```bash
wallet deploy-program target/risc0-guest/airdrop-program.bin
```

### 3. Generate distribution manifest

```bash
airdrop-cli generate \
  --csv recipients.csv \
  --token 0000000000000000000000000000000000000000000000000000000000000001 \
  --distributor <DISTRIBUTOR_ADDRESS> \
  --allocation 100000
```

### 4. Initialize on-chain

```bash
# Init the distributor (admin) account first
wallet auth-transfer init --account-id Public/<ADMIN>

# Serialize the init instruction
airdrop-cli serialize --instruction init \
  --merkle-root <MERKLE_ROOT> \
  --distributor <DISTRIBUTOR_ADDRESS> \
  --allocation 100000

# Submit (admin + state account both sign)
wallet call --program <PROGRAM_ID> --data <INIT_HEX> \
  --accounts "Public/<ADMIN>" "Public/<STATE_ACCOUNT>"
```

### 5. Claim

```bash
# Generate the recipient's claim proof
airdrop-cli proof --manifest distribution.json --address <MY_ADDRESS> > claim.json

# Serialize & submit the claim
airdrop-cli serialize --instruction claim --claim claim.json
wallet call --program <PROGRAM_ID> --data <CLAIM_HEX> \
  --accounts "Public/<ADMIN>" "Public/<STATE_ACCOUNT>"
```

## Privacy Guarantees

- **Eligibility is hidden**: On-chain state reveals only the Merkle root, not who is eligible
- **Claims are unlinkable**: Each claim produces a fresh nullifier; no on-chain link between claims
- **Amounts are hidden**: Individual allocation amounts are never revealed on-chain
- **Double-claim prevented**: Nullifier scheme prevents multiple claims with the same secret

## Security

The distribution state is stored in a program-owned account and can only be modified by the airdrop program. Eligibility is enforced inside the Risc0 guest:

1. `claim` computes `leaf = SHA256("airdrop_leaf" || recipient_address || amount || salt)`
2. Verifies the Merkle inclusion proof against the committed root
3. Computes `nullifier = SHA256("airdrop_nullifier" || nullifier_secret)` and asserts it is unused
4. Enforces `claimed_so_far + amount <= total_allocation` and `active == true`

Only recipients whose leaf is committed in the Merkle root can claim. The distribution manifest (salts + nullifier secrets) must remain private.

## Compute Units (public execution)

| Operation | CU |
|-----------|-----|
| Initialize | 82,576 |
| Claim | 751,132 |

Measured on LEZ testnet v0.2. Details: [docs/cu-costs.md](docs/cu-costs.md)

## Test

```bash
# Run SDK tests
cd sdk && cargo test --release
```

## Demo

- **Video:** [LP-0003 demo (Google Drive)](https://drive.google.com/file/d/1BNkKe2BEY6b1dEZeVZOIanqiSzvJrnXl/view) · [MP4 download](https://github.com/dhozil/logos-airdrop/releases/download/demo-v1/demo.video.mp4)
- See [docs/deployments.md](docs/deployments.md) for live deployments on LEZ testnet.

## License

MIT or Apache-2.0
