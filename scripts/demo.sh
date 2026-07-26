#!/usr/bin/env bash
set -euo pipefail

# End-to-end demo script for LP-0003 Private Airdrop Distributor
# Requires: LEZ wallet, airdrop program deployed, RISC0_DEV_MODE=0

PROGRAM_ID="${1:-<DEPLOYED_PROGRAM_ID>}"
TOKEN_PROGRAM_ID="${2:-<TOKEN_PROGRAM_ID>}"
DISTRIBUTOR="${3:-<DISTRIBUTOR_ADDRESS>}"

echo "=== LP-0003 Demo: Private Airdrop Distributor ==="
echo "Program ID: $PROGRAM_ID"
echo ""

# Step 1: Check wallet health
echo "1. Checking wallet health..."
wallet check-health

# Step 2: Build and deploy (if not already deployed)
if [ "$PROGRAM_ID" = "<DEPLOYED_PROGRAM_ID>" ]; then
  echo "2. Building program..."
  cd programs/airdrop
  RISC0_DEV_MODE=0 cargo risczero build --release
  echo "3. Deploying program..."
  PROGRAM_ID=$(wallet deploy-program target/risc0-guest/airdrop-program.bin)
  echo "Deployed at: $PROGRAM_ID"
  cd ../..
fi

# Step 3: Create sample recipients CSV
echo "4. Creating sample recipients..."
cat > /tmp/recipients.csv << 'EOF'
address,amount
0x6a756c69616e0000000000000000000000000000000000000000000000000001,1000
0x6a756c69616e0000000000000000000000000000000000000000000000000002,2000
0x6a756c69616e0000000000000000000000000000000000000000000000000003,3000
0x6a756c69616e0000000000000000000000000000000000000000000000000004,4000
0x6a756c69616e0000000000000000000000000000000000000000000000000005,5000
EOF

# Step 4: Generate distribution manifest
echo "5. Generating distribution manifest..."
airdrop-cli generate \
  --csv /tmp/recipients.csv \
  --token "$TOKEN_PROGRAM_ID" \
  --distributor "$DISTRIBUTOR" \
  --allocation 15000 \
  --output /tmp/distribution.json

MERKLE_ROOT=$(cat /tmp/distribution.json | python3 -c "import json,sys; print(json.load(sys.stdin)['config']['merkle_root'])")
echo "Merkle root: $MERKLE_ROOT"

# Step 5: Initialize on-chain (first distribution)
echo "6. Initializing first distribution on-chain..."
wallet public-tx \
  --program "$PROGRAM_ID" \
  --instruction initialize \
  --args "$MERKLE_ROOT" "$TOKEN_PROGRAM_ID" 15000 \
  --signer "$DISTRIBUTOR"

# Step 6: Claim as first recipient
echo "7. Claiming as first recipient..."
RECIPIENT="0x6a756c69616e0000000000000000000000000000000000000000000000000001"
CLAIM_DATA=$(airdrop-cli proof --manifest /tmp/distribution.json --address "$RECIPIENT")

NULLIFIER_SECRET=$(echo "$CLAIM_DATA" | python3 -c "import json,sys; print(json.load(sys.stdin)['nullifier_secret'])")
LEAF_INDEX=$(echo "$CLAIM_DATA" | python3 -c "import json,sys; print(json.load(sys.stdin)['leaf_index'])")
AMOUNT=$(echo "$CLAIM_DATA" | python3 -c "import json,sys; print(json.load(sys.stdin)['amount'])")
SALT=$(echo "$CLAIM_DATA" | python3 -c "import json,sys; print(json.load(sys.stdin)['salt'])")

# Need to serialize merkle_path for the wallet command
MERKLE_PATH=$(echo "$CLAIM_DATA" | python3 -c "
import json,sys
data = json.load(sys.stdin)
path = data['merkle_path']
# Format as hex string for LEZ wallet
result = ','.join(path)
print(result)
")

echo "Submitting private claim transaction..."
RISC0_DEV_MODE=0 wallet private-tx \
  --program "$PROGRAM_ID" \
  --instruction claim \
  --args "$NULLIFIER_SECRET" "$MERKLE_PATH" "$LEAF_INDEX" "$RECIPIENT" "$AMOUNT" "$SALT"

echo ""
echo "=== Demo Complete ==="
echo "First distribution initialized and claim submitted successfully."
