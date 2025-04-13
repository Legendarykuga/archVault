# ArchVault ⛓️ 🔥

A secure, gas-efficient, and user-centric vault system built for Arch Network — enabling time-locked deposits, multiple tokens, customizable lock durations, and emergency withdrawals with penalties.

---

## 🚀 Overview

**ArchVault** is a smart contract-based savings system that empowers users to lock multiple types of tokens (e.g., Bitcoin, Runes, and other CW20 tokens) for specified durations. Users can retrieve their tokens after the lock periods expire or perform emergency withdrawals with associated penalties. ArchVault enforces robust access controls, prioritizes gas optimization, and safeguards user funds through secure smart contract logic.  This project is written in Rust and uses CosmWasm.

---

## ✨ Features

-   ✅   **Multi-token deposit support** (e.g., BTC, Runes, CW20 tokens)
-   🔒   **User-defined lock periods**
-   ⏱️   **Time-locked secure withdrawals**
-   ➕   **Multiple deposits with separate durations**
-   🚨   **Emergency withdrawals with penalty**
-   🛡️   **Robust access control**
-   ⚡   **Gas-optimized deposit architecture**
-   🔐   **Fund safety and loss prevention mechanisms**
-   ✅  **CosmWasm Implementation** - Uses Rust and CosmWasm for smart contract functionality.

---

## 🛠️ How It Works

1.  **Deposit**:
    * Users execute the `deposit` message, specifying the `token`, `amount`, and `lock_duration`.
    * Funds are securely locked within the contract.
    * Each deposit is recorded separately, with its own unlock timestamp.

2.  **Withdraw**:
    * Users execute the `withdraw` message, providing the `index` of the deposit.
    * The contract verifies the lock period and sender's ownership before releasing the funds.

3.  **Emergency Withdraw**:
    * Users can execute the `emergency_withdraw` message with the deposit's `index` before the lock period expires.
    * A penalty (e.g., 10%) is deducted from the deposited amount, and the remaining amount is transferred to the user.  The penalty is sent to the treasury.

---

## 📦 Installation

### Prerequisites

* **Rust:** Ensure you have Rust installed.  You can install it from [https://rust-lang.org](https://rust-lang.org).
* **CosmWasm:** Set up your CosmWasm development environment.  This typically involves installing `cosmwasm-cli` and setting up a local blockchain.  Refer to the CosmWasm documentation for detailed instructions: [https://docs.cosmwasm.com/](https://docs.cosmwasm.com/)

### Build Instructions

1.  **Clone the repository:**

    ```bash
    git clone [https://github.com/yourusername/archVault.git](https://github.com/Legendarykuga/archVault.git)
    cd archVault
    ```

2.  **Build the contract:**

    ```bash
    cargo build --release
    ```

    This command compiles the Rust code into a Wasm binary, which can be deployed to a CosmWasm-enabled blockchain.

##   Usage

### Interacting with the Contract

The ArchVault contract exposes several functions (messages) that users can interact with.  You'll typically use a CosmWasm client (or a tool like `cw-cli`) to send these messages to the contract on the blockchain.

#### Contract Messages

* **`deposit { token, amount, lock_duration }`**
    * Deposits a specified amount of a token into a time-locked vault.
    * `token`:  The address of the token to deposit (for CW20 tokens).  For native tokens, use "".
    * `amount`: The amount of tokens to deposit.
    * `lock_duration`: The duration (in seconds) for which the tokens will be locked.

* **`withdraw { index }`**
    * Withdraws a previously deposited amount.
    * `index`: The index of the deposit to withdraw (starting from 0).  You can get the index from `get_my_deposits`.
    * The lock period for the specified deposit must have expired.

* **`emergency_withdraw { index }`**
    * Withdraws a previously deposited amount before the lock period has expired.
    * `index`: The index of the deposit to withdraw.
    * Incurs a penalty.

* **`get_my_deposits {}`**
    * Queries the contract to retrieve all deposits associated with the sender's address.
    * Returns an array of deposit objects, each containing the token, amount, lock duration, and status.

### Example Interaction (using `cw-cli`)

Here's an example of how to interact with the contract using the `cw-cli` tool.  You'll need to adapt this to your specific setup.

1.  **Instantiate the contract:**

    ```bash
    cw-cli wasm instantiate ./target/wasm32-unknown-unknown/release/arch_vault.wasm '{"owner": "<owner_address>"}' --label "arch-vault" --admin "<admin_address>" --from <deployer_address>
    ```

2.  **Deposit tokens:**

    ```bash
    cw-cli tx wasm execute <contract_address> '{"deposit": {"token": "<token_address>", "amount": "100", "lock_duration": 3600 }}' --from <sender_address> --amount 100<native_token_denom>
    ```
    * If you are depositing a native token, include the amount.  If it is a CW20 token, you will need to approve the contract to spend the tokens.

3.  **View deposits:**

    ```bash
     cw-cli query wasm contract-state <contract_address> '{"get_my_deposits": {}}'
    ```

4.  **Withdraw tokens:**

    ```bash
    cw-cli tx wasm execute <contract_address> '{"withdraw": {"index": 0}}' --from <sender_address>
    ```

5.  **Emergency withdraw:**

    ```bash
    cw-cli tx wasm execute <contract_address> '{"emergency_withdraw": {"index": 0}}' --from <sender_address>
    ```

## 📄 Smart Contract Safeguards

The ArchVault smart contract incorporates several safeguards to protect user funds and ensure proper operation:

* **Access Control**: Only the depositor can withdraw their funds.
* **Time Lock Enforcement**: Funds cannot be withdrawn before the lock period expires, except via the emergency withdrawal mechanism, which applies a penalty.
* **Emergency Fee Handling**: The penalty amount from emergency withdrawals is correctly calculated and transferred to the treasury.
* **Failsafe Checks**: The contract includes checks to prevent common errors, such as arithmetic underflows, incorrect token transfers, and unauthorized access.
* **CW20 Token Handling**: The contract correctly handles deposits and withdrawals of CW20 tokens, including necessary `transfer_from` and `transfer` calls.

---

## 🚀 Future Enhancements

* **More Flexible Penalty Mechanism:** Allow the penalty to be configurable.
* **Support for different lock mechanisms**: Explore different ways to lock funds.
* **Integration with other CosmWasm contracts.**
* **Testing**: Add more comprehensive tests.
