# Solution: LP-0003 — Private Allowlist / Airdrop Distributor

## Summary

A complete LP-0003 implementation: a private airdrop and allowlist primitive for LEZ where the distributor commits to a hidden eligibility set, and recipients claim without revealing their identity on-chain.

- **Distributor commits** to a Merkle root on-chain; individual addresses and amounts remain hidden.
- **Recipient claims** by submitting a LEZ transaction that verifies Merkle inclusion and produces a unique nullifier — no on-chain observer can link the claim to a specific address.
- **Double-claim prevention** via nullifier scheme: each `nullifier_secret` produces a unique `nullifier = SHA256("airdrop_nullifier" || secret)` that can only be claimed once.
- **Two live distributions on LEZ testnet with 43 confirmed claims** (22 + 21).
- **CLI tool** (`airdrop-cli`) for distribution setup, proof generation, and claim submission.

## Live Testnet Deployment

- **Program ID**: `26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721`
- **Deployment TX**: `7e2947e72a7720bd53eb40dd5691832a4264d8cc46c653aedd96cb7ca4e4d881`
- **Distribution A**: merkle root `8baa5eab…`, 22 claims (init TX `a373b051…`, block 959)
- **Distribution B**: merkle root `0d0fa522…`, 21 claims (init TX `2c0cee1d…`, block 996)

Full details in [`docs/deployments.md`](deployments.md).

## Repository

- **Repo:** `dhozil/logos-airdrop`
- **Branch:** `master`
- **Video Demo:** [download MP4](https://github.com/dhozil/logos-airdrop/releases/download/demo-v1/demo.video.mp4)

## Approach

Built on the Logos stack:

- **LEZ airdrop program** (`programs/airdrop/`) stores the distribution state in a program-owned account with the Merkle root, distributor, and allocation tracking. Three instructions: `initialize`, `claim`, and `close`. Runs inside the RISC0 zkVM; public transactions are re-executed by the sequencer.
- **Eligibility verification** happens entirely inside the LEZ program (the program IS the circuit). The `claim` instruction:
  1. Reads the distribution state (Merkle root, allocation tracking, nullifier set)
  2. Computes the leaf: `SHA256("airdrop_leaf" || recipient_address || amount || salt)`
  3. Verifies the Merkle inclusion proof against the committed root (20-depth path)
  4. Computes the nullifier and asserts it is unused (prevents double-claim)
  5. Updates `claimed_so_far` and appends the nullifier

## Security Properties

- Only recipients whose leaf is committed in the Merkle root can claim (salt is secret & random).
- Each `nullifier_secret` can be claimed exactly once.
- Allocation is capped by `total_allocation`; claims are blocked once `active` is false.
- Distribution state is stored in a program-owned account — cannot be modified externally.

## Compute Units (public execution)

| Operation | CU |
|-----------|-----|
| Initialize | 82,576 |
| Claim | 751,132 |

Full details in [`docs/cu-costs.md`](cu-costs.md).
