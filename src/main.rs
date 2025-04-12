use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{self, File};
use std::io::{self, Write};
use clap::{Command, Arg};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Deposit {
    amount: u128,
    unlock_time: u64,
    withdrawn: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vault {
    deposits: HashMap<String, Vec<Deposit>>,
}

impl Vault {
    fn new() -> Self {
        Vault {
            deposits: HashMap::new(),
        }
    }

    fn deposit(&mut self, user: String, amount: u128, _lock_seconds: u64) {
        let now = current_timestamp();
        let unlock_time = now; // Can use now + lock_seconds if desired

        let new_deposit = Deposit {
            amount,
            unlock_time,
            withdrawn: false,
        };

        self.deposits.entry(user).or_default().push(new_deposit);
    }

    fn withdraw(&mut self, user: &str) -> u128 {
        let now = current_timestamp();
        let mut total_withdrawn = 0;

        if let Some(user_deposits) = self.deposits.get_mut(user) {
            for deposit in user_deposits.iter_mut() {
                if !deposit.withdrawn && deposit.unlock_time <= now {
                    deposit.withdrawn = true;
                    total_withdrawn += deposit.amount;
                }
            }
        }

        total_withdrawn
    }

    fn emergency_withdraw(&mut self, user: &str, fee_percent: u8) -> u128 {
        let mut total_withdrawn = 0;

        if let Some(user_deposits) = self.deposits.get_mut(user) {
            for deposit in user_deposits.iter_mut() {
                if !deposit.withdrawn {
                    deposit.withdrawn = true;
                    let penalty = deposit.amount * fee_percent as u128 / 100;
                    total_withdrawn += deposit.amount - penalty;
                }
            }
        }

        total_withdrawn
    }

    fn load() -> io::Result<Self> {
        match fs::read("vault.bin") {
            Ok(data) => {
                let vault: Vault = bincode::deserialize(&data).unwrap_or_else(|_| Vault::new());
                Ok(vault)
            }
            Err(_) => Ok(Vault::new()),
        }
    }

    fn save(&self) -> io::Result<()> {
        let encoded = bincode::serialize(&self).expect("Serialization failed");
        let mut file = File::create("vault.bin")?;
        file.write_all(&encoded)?;
        Ok(())
    }

    fn view_deposits(&self, user: &str) {
        if let Some(user_deposits) = self.deposits.get(user) {
            if user_deposits.is_empty() {
                println!("No deposits found for user: {}", user);
            } else {
                println!("Deposits for user {}:", user);
                for (index, deposit) in user_deposits.iter().enumerate() {
                    println!(
                        "Deposit {}: Amount: {}, Unlock Time: {}, Withdrawn: {}",
                        index + 1,
                        deposit.amount,
                        deposit.unlock_time,
                        deposit.withdrawn
                    );
                }
            }
        } else {
            println!("No deposits found for user: {}", user);
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn main() {
    let matches = Command::new("Finance System")
        .version("1.0")
        .about("Manage user finances")
        .subcommand(
            Command::new("add_user")
                .about("Add a new user")
                .arg(
                    Arg::new("username")
                        .long("username")
                        .required(true)
                        .num_args(1)
                        .help("The username to add"),
                ),
        )
        .subcommand(
            Command::new("view_balance")
                .about("View balance")
                .arg(
                    Arg::new("username")
                        .long("username")
                        .required(true)
                        .num_args(1)
                        .help("The username to check balance for"),
                ),
        )
        .subcommand(
            Command::new("emergency_withdraw")
                .about("Emergency withdraw")
                .arg(
                    Arg::new("username")
                        .long("username")
                        .required(true)
                        .num_args(1)
                        .help("The username to perform emergency withdrawal for"),
                ),
        )
        .subcommand(
            Command::new("view_deposits")
                .about("View all deposits for a user")
                .arg(
                    Arg::new("username")
                        .long("username")
                        .required(true)
                        .num_args(1)
                        .help("The username to view deposits for"),
                ),
        )
        .get_matches();

    // Load persisted vault
    let mut vault = Vault::load().expect("Failed to load vault");

    if let Some(matches) = matches.subcommand_matches("add_user") {
        if let Some(username) = matches.get_one::<String>("username") {
            println!("Adding user: {}", username);
            vault.deposit(username.to_string(), 1000, 10);
        }
    } else if let Some(matches) = matches.subcommand_matches("view_balance") {
        if let Some(username) = matches.get_one::<String>("username") {
            println!("Viewing balance for: {}", username);
            let withdrawn = vault.withdraw(username);
            println!("Withdrawn amount: {}", withdrawn);
        }
    } else if let Some(matches) = matches.subcommand_matches("emergency_withdraw") {
        if let Some(username) = matches.get_one::<String>("username") {
            println!("Emergency withdrawal for: {}", username);
            let withdrawn = vault.emergency_withdraw(username, 10);
            println!("Emergency withdrawal amount: {}", withdrawn);
        }
    } else if let Some(matches) = matches.subcommand_matches("view_deposits") {
        if let Some(username) = matches.get_one::<String>("username") {
            println!("Viewing deposits for: {}", username);
            vault.view_deposits(username);
        }
    }

    // Save the updated state
    vault.save().expect("Failed to save vault");
}
