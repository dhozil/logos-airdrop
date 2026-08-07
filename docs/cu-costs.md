# Compute Unit Costs

Measured on LEZ testnet v0.2 (public execution, Risc0 guest `airdrop-v2-ci.bin`).

Compute Units (CU) are the Risc0 execution cycle count measured by `default_executor` (the same path the sequencer uses to validate public transactions).

## Operations

| Operation | CU (cycles) | Notes |
|-----------|-------------|-------|
| Initialize Distribution | 82,576 | Stores 32-byte merkle root + distributor + allocation |
| Claim | 751,132 | Verifies 20-level merkle proof + nullifier + allocation update |
| Close Distribution | ~10,000 | Marks distribution inactive |

## Merkle Proof Verification (in-circuit)

A 20-depth merkle path verification dominates the claim cost. Each `hash_pair` (two SHA-256 of 32-byte inputs) costs ~35,000-37,000 cycles; the claim performs 20 of them (~750K cycles total).

## Performance Benchmarks

| Component | Time | Notes |
|-----------|------|-------|
| Merkle tree generation (22 leaves, depth 20) | <10ms | SDK, single-threaded |
| Claim proof generation (private exec) | ~60s | Real Risc0 STARK proof (est.) |
| Claim public execution (on-chain) | ~2-3s | Sequencer re-executes the guest |

**Note**: LEZ testnet v0.2 compute budget may change. Measurements from `testnet.lez.logos.co`, guest commit `d9768e8`.
