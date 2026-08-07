#!/usr/bin/env bash
set -euo pipefail

# End-to-end demo script for LP-0003 Private Airdrop Distributor
# Requires: LEZ wallet CLI v0.2.2, airdrop program deployed on testnet

PROGRAM_ID="${1:-26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721}"
ADMIN="${2:-Public/AKy1PsJFCR7LBQMdCjH8G3GYmNGcc3gs293bPGEboKSs}"
STATE="${3:-Public/BxajpycZ2zbodcxLT6jLkgnnbiqeA4VifNfGF2RDT6X5}"
CLI="target/release/airdrop-cli"
WALLET="wallet"

echo "=== LP-0003 Demo: Private Airdrop Distributor ==="
echo "Program ID: $PROGRAM_ID"
echo ""

# Step 1: Check wallet health
echo "1. Checking wallet health..."
$WALLET check-health

# Step 2: Create sample recipients CSV
echo "2. Creating sample recipients..."
cat > /tmp/recipients.csv << 'EOF'
address,amount
1111111111111111111111111111111111111111111111111111111111111111,1000
1111111111111111111111111111111111111111111111111111111111111112,1000
1111111111111111111111111111111111111111111111111111111111111113,1000
1111111111111111111111111111111111111111111111111111111111111114,1000
1111111111111111111111111111111111111111111111111111111111111115,1000
EOF

# Step 3: Generate distribution manifest
echo "3. Generating distribution manifest..."
$CLI generate \
  --csv /tmp/recipients.csv \
  --token 0000000000000000000000000000000000000000000000000000000000000001 \
  --distributor 8a94f6c7f2fa5b430dd5a5ce0dd525152c778a31cb12cfaf4c0231e60af99d94 \
  --allocation 5000 \
  --output /tmp/distribution.json

MERKLE_ROOT=$($CLI status --manifest /tmp/distribution.json 2>/dev/null | grep -oE "Merkle Root: [0-9a-f]+" | awk '{print $3}')
echo "Merkle root: $MERKLE_ROOT"

# Step 4: Serialize init
echo "4. Serializing init instruction..."
INIT_HEX=$($CLI serialize --instruction init \
  --merkle-root "$MERKLE_ROOT" \
  --distributor 8a94f6c7f2fa5b430dd5a5ce0dd525152c778a31cb12cfaf4c0231e60af99d94 \
  --allocation 5000)

# Step 5: Init on-chain (admin + state sign)
echo "5. Initializing distribution..."
$WALLET call --program "$PROGRAM_ID" --data "$INIT_HEX" \
  --accounts "$ADMIN" "$STATE"

# Step 6: Prepare & submit a claim
echo "6. Preparing claim..."
ADDR="1111111111111111111111111111111111111111111111111111111111111111"
$CLI proof --manifest /tmp/distribution.json --address "$ADDR" > /tmp/claim.json
CLAIM_HEX=$($CLI serialize --instruction claim --claim /tmp/claim.json)
echo "7. Submitting claim..."
$WALLET call --program "$PROGRAM_ID" --data "$CLAIM_HEX" \
  --accounts "$ADMIN" "$STATE"

echo ""
echo "=== Demo complete ==="
