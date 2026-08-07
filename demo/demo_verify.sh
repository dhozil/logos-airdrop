#!/bin/bash
# ============================================================
# LP-0003 Demo: On-Chain Verification (non-destructive)
# Shows proof of the live deployment without changing state
# ============================================================
set -euo pipefail

WALLET="/root/.cargo/bin/wallet-v022"
PID="26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721"
PID_BASE58="3cdWPWHHYfy6V6qKbjxqVHrjDmnsLKZ87Qr3wYhguDmi"
DEPLOY_TX="7e2947e72a7720bd53eb40dd5691832a4264d8cc46c653aedd96cb7ca4e4d881"
ADMIN="Public/AKy1PsJFCR7LBQMdCjH8G3GYmNGcc3gs293bPGEboKSs"
STATE_A="Public/BxajpycZ2zbodcxLT6jLkgnnbiqeA4VifNfGF2RDT6X5"
STATE_B="Public/89555exkSkc1zuAZFzt2DmJn46gjFxSiw9DDveV5dAsm"

echo "============================================================"
echo "LP-0003 DEMO: On-Chain Verification"
echo "Program ID (hex):     $PID"
echo "Program ID (base58):  $PID_BASE58   (= same program)"
echo "============================================================"

echo ""
echo "=== 1. Wallet health ==="
$WALLET check-health

echo ""
echo "=== 2. Program deployment on-chain ==="
$WALLET chain-info transaction --hash $DEPLOY_TX

echo ""
echo "=== 3. Distribution A state ==="
echo "# program_owner ($PID_BASE58) is the airdrop program (base58 of $PID)"
$WALLET account get --account-id $STATE_A

echo ""
echo "=== 4. Distribution B state ==="
$WALLET account get --account-id $STATE_B

echo ""
echo "=== 5. Current chain height ==="
$WALLET chain-info current-block-id

echo ""
echo "=== DEMO VERIFICATION COMPLETE ==="
echo "Distribution A: merkle 8baa5eab... (22 claims)"
echo "Distribution B: merkle 0d0fa522... (21 claims)"
echo "Total: 2 distributions, 43 claims"
echo "============================================================"
