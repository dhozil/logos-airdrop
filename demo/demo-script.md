# LP-0003 Video Demo - Production Script (English)

**Target duration**: 3-4 minutes
**Format**: Screen recording + voiceover
**Tools**: OBS Studio (free), terminal, SVG diagrams (included)

---

## 🎬 Storyboard

### Scene 1: Opening + Architecture (0:00 - 0:40)
- **Visual**: Diagram `airdrop-architecture.svg`
- **Narration**:
  > "LP-0003 is a private airdrop for the Logos Execution Zone. The distributor builds a Merkle tree from the recipient list offline, then commits only the Merkle root on-chain — so who is eligible remains hidden. Recipients claim with a proof verified inside the Risc0 zkVM, producing a unique nullifier so nobody can double-claim."

### Scene 2: Program Live on Testnet (0:40 - 1:10)
- **Visual**: Terminal - run `demo_verify.sh` part 1-2
- **Narration**:
  > "The program is deployed live on the LEZ testnet. Here's the transaction — a 210KB Risc0 guest bytecode running in the zkVM. The program ID is `26d7fafc...`"

### Scene 3: Two Live Distributions (1:10 - 1:50)
- **Visual**: Terminal - `demo_verify.sh` part 3-4 (state accounts)
- **Narration**:
  > "We have 2 live distributions. Distribution A: Merkle root `8baa5eab...` with 22 claims. Distribution B: Merkle root `0d0fa522...` with 21 claims. Both state accounts are owned by the airdrop program. That's 43 claims in total."

### Scene 4: Claim Flow + Security (1:50 - 2:40)
- **Visual**: Diagram `airdrop-claimflow.svg`
- **Narration**:
  > "Here's the claim flow. Inside the zkVM, the guest: (1) computes the leaf from address + amount + salt, (2) verifies the Merkle path against the committed root, (3) computes the nullifier and checks it hasn't been used, (4) updates the state. Because the salt is a random secret, only committed recipients can claim. The nullifier prevents double-claims. Allocation is capped."

### Scene 5: On-Chain Verification (2:40 - 3:20)
- **Visual**: Terminal - run `demo_verify.sh` fully (especially state data + chain height)
- **Narration**:
  > "Here's the on-chain proof: Distribution A's state shows claimed_so_far = 47,300 with 22 nullifiers. Distribution B: 21,000 with 21 nullifiers. The chain is currently at block 1048."

### Scene 6: Closing (3:20 - 3:40)
- **Narration**:
  > "The code, CI, and deployment are all ready. CU costs: Initialize 82,576 cycles, Claim 751,132 cycles. Full details are in the repo `dhozil/logos-airdrop`. Thank you."

---

## 📁 Included Files

| File | Purpose |
|------|---------|
| `demo_verify.sh` | On-chain verification script (ready to record) |
| `airdrop-architecture.svg` | Architecture diagram (Scene 1) |
| `airdrop-claimflow.svg` | Claim flow + security diagram (Scene 4) |

## 🛠️ How to use

1. Open a WSL terminal
2. Run `bash demo/demo_verify.sh` - record the full output
3. Open the 2 SVGs in a browser for Scenes 1 & 4
4. Record with OBS, voiceover following the narration

## 🔑 Key Data (for PR #44)

- **Program ID**: `26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721`
- **Deployment TX**: `7e2947e72a7720bd53eb40dd5691832a4264d8cc46c653aedd96cb7ca4e4d881`
- **Dist A init TX**: `a373b0512011ea9a10fe8243cb6fc96c2645fb324ab70426dd19898de87149c0` (block 959)
- **Dist B init TX**: `2c0cee1d034fa9e6561a59634275837784f81722d9b03faea86bb3c9ecc87686` (block 996)
- **State A**: `BxajpycZ2zbodcxLT6jLkgnnbiqeA4VifNfGF2RDT6X5` (22 claims, 47,300)
- **State B**: `89555exkSkc1zuAZFzt2DmJn46gjFxSiw9DDveV5dAsm` (21 claims, 21,000)
- **Admin**: `AKy1PsJFCR7LBQMdCjH8G3GYmNGcc3gs293bPGEboKSs`
