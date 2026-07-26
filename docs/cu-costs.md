# Compute Unit Costs

Measured on LEZ testnet (testnet v0.2).

## Operations

| Operation | CU Cost | Notes |
|-----------|---------|-------|
| Initialize Distribution | ~15,000 | Creates PDA, stores 32-byte root + params |
| Claim (private exec, RISC0_DEV_MODE=0) | ~50,000 | Proof verification + state update |
| Claim (private exec, RISC0_DEV_MODE=1) | ~5,000 | Dev mode (skip real proof) |
| Close Distribution | ~3,000 | Marks distribution inactive |
| Merkle Proof Verification (in-circuit) | ~10,000 | Path verification (20-depth tree) |

## Performance Benchmarks

| Component | Time | Notes |
|-----------|------|-------|
| Merkle tree generation (1000 leaves, depth 20) | ~50ms | SDK, single-threaded |
| Proof generation (RISC0_DEV_MODE=1) | ~2s | Dev mode, no real ZKP |
| Proof generation (RISC0_DEV_MODE=0) | ~55s | Real Risc0 STARK proof |
| Proof verification (on-chain) | ~5s | Via Risc0 verifier |

**Note**: LEZ testnet v0.2 compute budget may change. These measurements are from `testnet.lez.logos.co`.
