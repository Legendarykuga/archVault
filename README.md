🚀 Installation
1. Clone the Repository
bash
Copy
Edit
git clone https://github.com/Legendarykuga/vault_savings.git
cd vault_savings
2. Build the Project
Use the release profile for optimized performance:

bash
Copy
cargo build --release
3. Run the Program
Use the following syntax to execute commands:

bash
Copy
cargo run -- <COMMAND> --username <YOUR_USERNAME>
🛠️ Available Commands
🔹 add_user
Creates a new user and deposits 1000 tokens with a time lock.

⏳ Note: Lock duration can be adjusted in the source code (main.rs).

bash
Copy
cargo run -- add_user --username alice
🔹 view_balance
Withdraws all matured deposits (i.e., deposits whose unlock time has passed).

bash
Copy
cargo run -- view_balance --username alice
🔹 emergency_withdraw
Withdraws all funds immediately with a 10% penalty applied to each deposit.

bash
Copy
cargo run -- emergency_withdraw --username alice
🔹 view_deposits
Displays all deposit records for the specified user without withdrawing any funds.

bash
Copy
cargo run -- view_deposits --username alice
📁 File Structure
bash
Copy
vault_savings/
├── src/
│   └── main.rs        # Core logic for the vault system
├── vault.bin          # Binary file storing serialized deposit data
├── Cargo.toml         # Project configuration and dependencies
└── README.md          # Project documentation