# LP-0003 Video Demo - Production Script

**Durasi target**: 3-4 menit
**Format**: Screen recording + voiceover
**Alat**: OBS Studio (free), terminal, diagram SVG (disertakan)

---

## 🎬 Storyboard

### Scene 1: Pembukaan + Arsitektur (0:00 - 0:40)
- **Visual**: Diagram `airdrop-architecture.svg`
- **Narasi**:
  > "LP-0003 adalah private airdrop untuk LEZ. Distributor membangun Merkle tree dari daftar recipient secara offline, lalu commit hanya merkle root ke on-chain — siapa yang eligible tetap tersembunyi. Recipient claim dengan proof yang diverifikasi di dalam Risc0 zkVM, menghasilkan nullifier unik sehingga tidak ada yang bisa double-claim."

### Scene 2: Program Live di Testnet (0:40 - 1:10)
- **Visual**: Terminal - jalankan `demo_verify.sh` bagian 1-2
- **Narasi**:
  > "Program sudah ter-deploy live di LEZ testnet. Ini transaksinya — 210KB bytecode Risc0 guest yang berjalan di zkVM. Program ID: `26d7fafc...`"

### Scene 3: Dua Distribusi Live (1:10 - 1:50)
- **Visual**: Terminal - `demo_verify.sh` bagian 3-4 (state accounts)
- **Narasi**:
  > "Ada 2 distribusi live. Distribution A: merkle root `8baa5eab...` dengan 22 claims. Distribution B: merkle root `0d0fa522...` dengan 21 claims. Kedua state account di-owner oleh airdrop program. Total 43 claims."

### Scene 4: Flow Claim + Keamanan (1:50 - 2:40)
- **Visual**: Diagram `airdrop-claimflow.svg`
- **Narasi**:
  > "Ini flow claim. Di dalam zkVM, guest: (1) compute leaf dari address + amount + salt, (2) verify merkle path terhadap root yang dikomit, (3) compute nullifier dan pastikan belum dipakai, (4) update state. Karena salt adalah rahasia acak, hanya recipient yang ter-commit yang bisa klaim. Nullifier mencegah double-claim. Allocation di-cap."

### Scene 5: Verifikasi On-Chain (2:40 - 3:20)
- **Visual**: Terminal - `demo_verify.sh` lengkap (khususnya state data + chain height)
- **Narasi**:
  > "Bukti on-chain: state Distribution A menunjukkan claimed_so_far=47300 dengan 22 nullifiers. Distribution B: 21000 dengan 21 nullifiers. Chain saat ini di block 1048."

### Scene 6: Penutup (3:20 - 3:40)
- **Narasi**:
  > "Kode, CI, dan deployment sudah siap. CU cost: Initialize 82,576 cycles, Claim 751,132 cycles. Detail ada di repo `dhozil/logos-airdrop`. Terima kasih."

---

## 📁 File yang disiapkan

| File | Kegunaan |
|------|----------|
| `demo_verify.sh` | Script verifikasi on-chain (siap di-record) |
| `airdrop-architecture.svg` | Diagram arsitektur (Scene 1) |
| `airdrop-claimflow.svg` | Diagram flow claim + security (Scene 4) |

## 🛠️ Cara pakai

1. Buka terminal WSL
2. Jalankan `bash /mnt/d/logos/demo_verify.sh` - record seluruh output
3. Buka 2 SVG di browser untuk Scene 1 & 4
4. Rekam dengan OBS, voiceover sesuai narasi

## 🔑 Data kunci (untuk PR #44)

- **Program ID**: `26d7fafc8e6d6ce035979a9b5c692e5367e3e2ec6123116b1f75edef13bd8721`
- **Deployment TX**: `7e2947e72a7720bd53eb40dd5691832a4264d8cc46c653aedd96cb7ca4e4d881`
- **Dist A init TX**: `a373b0512011ea9a10fe8243cb6fc96c2645fb324ab70426dd19898de87149c0` (block 959)
- **Dist B init TX**: `2c0cee1d034fa9e6561a59634275837784f81722d9b03faea86bb3c9ecc87686` (block 996)
- **State A**: `BxajpycZ2zbodcxLT6jLkgnnbiqeA4VifNfGF2RDT6X5` (22 claims, 47300)
- **State B**: `89555exkSkc1zuAZFzt2DmJn46gjFxSiw9DDveV5dAsm` (21 claims, 21000)
- **Admin**: `AKy1PsJFCR7LBQMdCjH8G3GYmNGcc3gs293bPGEboKSs`
