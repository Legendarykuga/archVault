# vault_savings
This project is a finance management system where users can deposit and withdraw money based on time constraints. It supports emergency withdrawals with a penalty.
## Features
- Add new users
- View balances
- Regular withdrawals
- Emergency withdrawals with penalty
## Build and Run
### Prerequisites
- Rust (Install via https://rustup.rs)
- Cargo (Comes with Rust)
```bash
cargo build --release
Installation
1. Clone the repository
git clone https://github.com/Legendarykuga/vault_savings.git
cd vault_savings
2. Run the program 
cargo run -- <COMMAND> --username <YOUR_USERNAME>
3. Available Commands
add_user – Adds a new user and deposits 1000 tokens (lock time can be modified in code).
cargo run -- add_user --username alice
view_balance – Withdraw available funds if unlock time has passed.
cargo run -- view_balance --username alice
emergency_withdraw – Withdraw early with a 10% penalty.
cargo run -- emergency_withdraw --username alice
view_deposits – View your deposit info without withdrawing.
cargo run -- view_deposits --username alice
4. File structure
vault_savings/
├── src/
│   └── main.rs        # Main Rust logic (Vault system)
├── vault.bin          # Binary file where deposits are stored (serialized)
├── Cargo.toml         # Project dependencies and metadata
└── README.md          # Project documentation

5. Author
Kuga
GitHub: @Legendarykuga
6. License 
This project is licensed under the MIT License. See the LICENSE file for details.
---
