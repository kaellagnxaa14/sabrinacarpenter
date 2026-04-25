# AttendPay
Attendance-based student reward system powered by Stellar.

---

## 🧩 Problem
A public high school teacher in Manila manually records attendance using paper or spreadsheets, leading to inaccurate records that cause students to miss government stipends worth ₱500–₱1,000/month despite actually attending classes.

---

## 💡 Solution
AttendPay records student attendance on-chain using Soroban smart contracts; when a student checks in via QR scan, their attendance is permanently logged and can trigger XLM-based rewards, ensuring transparent, tamper-proof eligibility for stipends.

---

## 👥 Target Users
- Public high school students (ages 15–18)  
- Schools in Metro Manila and other SEA regions  
- Teachers and administrators managing attendance  
- Government or NGOs distributing student incentives  

---

## ⚙️ How It Works (MVP Flow)
1. Student scans a QR code upon entering class  
2. Frontend sends attendance data to Soroban contract  
3. Contract records attendance (student wallet + date)  
4. System verifies eligibility (e.g. attendance count)  
5. Reward is triggered (XLM payout or logged event)  

⏱️ Full demo flow: **under 2 minutes**

---

## 🚀 Core Features
- 📌 On-chain attendance tracking (tamper-proof)
- 💰 Attendance-based reward distribution (XLM)
- 🔍 Instant verification of attendance records
- ⚡ Fast and low-cost transactions via Stellar
- 🔗 Wallet-based identity (no manual encoding)

---

## 🛠️ Stellar Features Used
- **Soroban Smart Contracts** — attendance logic and validation  
- **XLM Transfers** — reward distribution (simulated in MVP)  
- **Trustlines** — optional for token-based rewards  

---

## 🧪 Example Use Case
Juan, a Grade 11 student in Quezon City:
- Attends school regularly but sometimes isn’t marked present  
- Uses AttendPay QR system daily  
- Attendance is recorded on-chain instantly  
- After meeting required days, he receives XLM reward  
- No disputes, no delays, no missing records  

---

## 📅 Development Timeline (5 Days)
- **Day 1–2:** Smart contract (attendance + validation)  
- **Day 3:** QR code scanning interface (frontend)  
- **Day 4:** Reward trigger logic integration  
- **Day 5:** Demo polish + testing  

---

## 🧰 Prerequisites
- Rust (latest stable)
- Soroban CLI (v22+)
- Stellar testnet account

---

## 🏗️ Build
```bash
soroban contract build


✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/29974420e3d7e590e1a557d9d6d6fb6d475d927ea58740e3391217e85c32c381
🔗 https://lab.stellar.org/r/testnet/contract/CAQRJK7XZJ6SUL3MITM55EJF3ZUPM4MP3SR27DSJQDPSHNNJW3SPFEJ3
✅ Deployed!
CAQRJK7XZJ6SUL3MITM55EJF3ZUPM4MP3SR27DSJQDPSHNNJW3SPFEJ3