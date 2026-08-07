# Private Airdrop / Allowlist Protocol

## Overview

This protocol enables private token airdrops on the Logos Execution Zone (LEZ). A distributor commits to an eligibility set on-chain without revealing individual addresses, and recipients claim their allocation without revealing which address they control.

## Protocol Flow

### Phase 1: Distribution Setup (Off-chain + On-chain)

1. **Prepare recipient list**: Distributor compiles list of `(recipient_address, amount)` pairs
2. **Generate secrets**: For each recipient, generate a random 32-byte salt and nullifier secret
3. **Build Merkle tree**: Each leaf = `SHA256("airdrop_leaf" || recipient_address || amount || salt)`
4. **Initialize on-chain**: Distributor calls `initialize(merkle_root, distributor, total_allocation)` on the airdrop program, which:
   - Claims a state account (owned by the airdrop program)
   - Stores the Merkle root, distributor, allocation, and empty nullifier set

### Phase 2: Claim (Public Execution)

1. **Prepare claim data**: Recipient (or distributor) assembles claim input:
   - `nullifier_secret`: The secret assigned to the recipient
   - `merkle_path`: Sibling hashes for the Merkle inclusion proof
   - `leaf_index`: Position of the recipient's leaf in the tree
   - `recipient_address`: The recipient's LEZ address
   - `amount`: Their allocation amount
   - `salt`: Their unique salt

2. **Submit claim**: A public LEZ transaction executes the airdrop program on-chain (Risc0 zkVM re-execution by the sequencer). The guest:
   - Computes `leaf = SHA256("airdrop_leaf" || recipient_address || amount || salt)`
   - Verifies Merkle inclusion: `verify_path(leaf, merkle_path, leaf_index) == stored_root`
   - Computes `nullifier = SHA256("airdrop_nullifier" || nullifier_secret)`
   - Asserts the nullifier is unused and `claimed_so_far + amount <= total_allocation`
   - Updates the distribution state (adds nullifier, increments `claimed_so_far`)

## On-Chain State

The distribution state is stored in a program-owned account:

```
merkle_root:      [u8; 32]
distributor:      [u8; 32]
total_allocation: u64
claimed_so_far:   u64
active:           bool
nullifiers:       Vec<[u8; 32]>
```

Only the airdrop program (owner) can modify this account.

## Privacy Model

| Participant | Knows |
|------------|-------|
| On-chain observer | Merkle root, distribution parameters, total claimed so far, nullifier set |
| Distributor | Full recipient list, all secrets, can see which addresses claimed |
| Recipient | Their own allocation and proof |
| Other recipients | Nothing about other recipients |

**Note**: The distributor inherently knows all recipients (they created the list). The privacy guarantee is that:
- No on-chain observer can determine which address claimed
- Each claim reveals only a nullifier (unlinkable across claims)
- Recipients cannot see each other's eligibility

## Circuit Design

The eligibility verification runs inside the LEZ program (Risc0 guest):

```
Public inputs:  self_program_id, caller_program_id, pre_states, instruction_words
Computed:       leaf = SHA256("airdrop_leaf" || recipient_address || amount || salt)
                root = verify_path(leaf, merkle_path, leaf_index)
                nullifier = SHA256("airdrop_nullifier" || nullifier_secret)
Constraints:    root == stored_merkle_root (from state account)
                nullifier not in stored nullifier set
                claimed_so_far + amount <= total_allocation
                active == true
Post-state:     claimed_so_far += amount; nullifiers.push(nullifier)
```

## Security Considerations

### Eligibility Enforcement
Only a leaf committed in the Merkle root can produce a valid inclusion proof. Because the salt is a random secret known only to the distributor/recipient, an outsider cannot forge a leaf for an arbitrary address.

### Double-Claim Prevention
Each `nullifier_secret` produces a unique nullifier stored in the state. A second claim with the same secret is rejected (`nullifier already claimed`).

### Allocation Cap
`claimed_so_far + amount <= total_allocation` prevents draining beyond the committed pool.

### Front-running Protection
A claim reveals only a nullifier, not the recipient address. Only the holder of the nullifier secret can produce a valid claim.

### Replay Across Distributions
Each distribution has its own state account and nullifier set. A claim from one distribution cannot be replayed on another (fresh nullifier set).

### Trust Anchor
The distribution manifest (salts + nullifier secrets) must remain private. Anyone with the full manifest can claim all allocations.

## Integration Guide

### Prerequisites
- LEZ wallet CLI v0.2.2 configured for testnet
- Airdrop program deployed on LEZ testnet
- Recipient address list in CSV format

### Steps
```bash
# 1. Generate distribution manifest
airdrop-cli generate \
  --csv recipients.csv \
  --token <TOKEN_PROGRAM_ID> \
  --distributor <DISTRIBUTOR_ADDRESS> \
  --allocation 100000

# 2. Initialize the admin (distributor) account
wallet auth-transfer init --account-id Public/<ADMIN>

# 3. Serialize & submit init (admin + state sign)
airdrop-cli serialize --instruction init \
  --merkle-root <MERKLE_ROOT> \
  --distributor <DISTRIBUTOR_ADDRESS> \
  --allocation 100000
wallet call --program <PROGRAM_ID> --data <INIT_HEX> \
  --accounts "Public/<ADMIN>" "Public/<STATE_ACCOUNT>"

# 4. Prepare & submit claims
airdrop-cli proof --manifest distribution.json --address <RECIPIENT_ADDRESS> > claim.json
airdrop-cli serialize --instruction claim --claim claim.json
wallet call --program <PROGRAM_ID> --data <CLAIM_HEX> \
  --accounts "Public/<ADMIN>" "Public/<STATE_ACCOUNT>"
```
