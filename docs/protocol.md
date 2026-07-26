# Private Airdrop / Allowlist Protocol

## Overview

This protocol enables private token airdrops on the Logos Execution Zone (LEZ). A distributor commits to an eligibility set on-chain without revealing individual addresses, and recipients claim their allocation without revealing which address they control.

## Protocol Flow

### Phase 1: Distribution Setup (Off-chain + On-chain)

1. **Prepare recipient list**: Distributor compiles list of `(recipient_address, amount)` pairs
2. **Generate secrets**: For each recipient, generate a random 32-byte salt and nullifier secret
3. **Build Merkle tree**: Each leaf = `SHA256("airdrop_leaf" || recipient_address || amount || salt)`
4. **Initialize on-chain**: Distributor calls `initialize(merkle_root, token_program_id, total_allocation)` on the airdrop program, which:
   - Creates a distribution state account (PDA)
   - Stores the Merkle root and distribution parameters
   - Transfers tokens from distributor to the program escrow

### Phase 2: Claim (Private Execution)

1. **Prepare claim data**: Recipient generates claim input:
   - `nullifier_secret`: The secret assigned to them
   - `merkle_path`: Sibling hashes for Merkle inclusion proof
   - `leaf_index`: Position of their leaf in the tree
   - `recipient_address`: Their LEZ address
   - `amount`: Their allocation amount
   - `salt`: Their unique salt

2. **Generate proof**: Recipient runs the airdrop program locally (private execution), producing a Risc0 ZK proof that:
   - Computes the leaf hash from their private inputs
   - Verifies Merkle inclusion (leaf + path → root matches on-chain root)
   - Computes nullifier = `SHA256("airdrop_nullifier" || nullifier_secret)`
   - Commits to (nullifier, recipient_address, amount) as public outputs

3. **Submit claim**: Recipient submits the Risc0 proof to the LEZ sequencer, which:
   - Verifies the ZK proof
   - Checks the nullifier hasn't been used (double-claim prevention)
   - Transfers tokens from the distribution escrow to the recipient
   - Updates the claimed amount in distribution state

### Privacy Model

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
Public inputs:  (none - all inputs are private to the user's execution)
Private inputs: nullifier_secret, merkle_path, leaf_index, recipient_address, amount, salt
Computed:       leaf = SHA256("airdrop_leaf" || recipient_address || amount || salt)
                root = verify_path(leaf, merkle_path, leaf_index)
                nullifier = SHA256("airdrop_nullifier" || nullifier_secret)
Constraints:    root == stored_merkle_root (checked on-chain from state)
Post-state:     claimed_so_far += amount
```

### Nullifier Scheme

- Each recipient has a unique `nullifier_secret` (32 bytes random)
- Nullifier = `SHA256("airdrop_nullifier" || nullifier_secret)`
- On-chain nullifier set prevents double-claims
- Nullifiers are public but unlinkable: an observer cannot determine which recipient a nullifier belongs to

## Security Considerations

### Double-Claim Prevention
The on-chain verifier maintains an implicit nullifier set via the `claimed_so_far` counter and the unique nullifier output. Each nullifier can only be claimed once (enforced by the ZK proof's unique nullifier output).

### Front-running Protection
Since claims reveal only the nullifier (not the recipient address), an observer cannot front-run a specific claim. Only the holder of the nullifier secret can produce a valid proof.

### Replay Attacks
Each proof is bound to a specific distribution instance via the on-chain state account. A proof from one distribution cannot be reused on another.

## Integration Guide

### Prerequisites
- LEZ wallet configured and funded
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

# 2. Initialize on-chain
wallet public-tx --program <AIRDROP_PROGRAM> \
  --instruction initialize \
  --args <merkle_root> <token_id> <total_allocation>

# 3. Recipient: prepare claim data
airdrop-cli proof \
  --manifest distribution.json \
  --address <RECIPIENT_ADDRESS>

# 4. Recipient: submit claim (private execution)
wallet private-tx --program <AIRDROP_PROGRAM> \
  --instruction claim \
  --args <nullifier_secret> <merkle_path> <leaf_index> <address> <amount> <salt>
```
