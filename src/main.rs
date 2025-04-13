// Arch Network Vault CLI (Mock) - Time-Locked Vaults
const inquirer = require('inquirer');
const chalk = require('chalk');

let vaults = [];

function getUserVaults(address) {
  return vaults.filter(v => v.owner === address);
}

async function mainMenu() {
  console.clear();
  console.log(chalk.cyan.bold('\n=== ArchVault: Time-Locked Vault System ==='));

  const { address } = await inquirer.prompt({
    name: 'address',
    type: 'input',
    message: 'Enter your wallet address:',
    validate: input => input ? true : 'Address cannot be empty'
  });

  while (true) {
    const { action } = await inquirer.prompt({
      name: 'action',
      type: 'list',
      message: `Welcome ${chalk.green(address)}! What would you like to do?`,
      choices: [
        'Deposit Funds',
        'Withdraw Funds',
        'Emergency Withdraw',
        'View My Vaults',
        'Exit'
      ]
    });

    if (action === 'Deposit Funds') {
      await handleDeposit(address);
    } else if (action === 'Withdraw Funds') {
      await handleWithdraw(address);
    } else if (action === 'Emergency Withdraw') {
      await handleEmergency(address);
    } else if (action === 'View My Vaults') {
      displayVaults(getUserVaults(address));
      await inquirer.prompt({ name: 'next', type: 'input', message: 'Press Enter to return to menu...' });
    } else {
      console.log(chalk.yellow('Goodbye!'));
      process.exit(0);
    }
  }
}

async function handleDeposit(owner) {
  const { token, amount, lockPeriod } = await inquirer.prompt([
    {
      name: 'token',
      type: 'list',
      message: 'Select token to deposit:',
      choices: ['BTC', 'RUNES', 'ETH']
    },
    {
      name: 'amount',
      type: 'input',
      message: 'Enter amount to deposit:',
      validate: input => isNaN(input) || input <= 0 ? 'Enter a valid amount' : true
    },
    {
      name: 'lockPeriod',
      type: 'input',
      message: 'Enter lock period (in seconds):',
      validate: input => isNaN(input) || input <= 0 ? 'Enter a valid time' : true
    }
  ]);

  const { confirm } = await inquirer.prompt({
    name: 'confirm',
    type: 'confirm',
    message: `Lock ${amount} ${token} for ${lockPeriod} seconds?`,
  });

  if (!confirm) {
    console.log(chalk.red('Deposit cancelled.'));
    return;
  }

  const timestamp = Math.floor(Date.now() / 1000);
  vaults.push({
    owner,
    token,
    amount: parseFloat(amount),
    lockPeriod: parseInt(lockPeriod),
    timestamp
  });

  console.log(chalk.green('Deposit successful! Funds are now time-locked.'));
}

async function handleWithdraw(address) {
  const userVaults = getUserVaults(address).filter(v => !v.withdrawn);

  if (userVaults.length === 0) {
    console.log(chalk.red('No withdrawable vaults found.'));
    return;
  }

  const { index } = await inquirer.prompt({
    name: 'index',
    type: 'list',
    message: 'Select a vault to withdraw:',
    choices: userVaults.map((v, i) => ({
      name: `${v.amount} ${v.token} (Locked for ${v.lockPeriod}s)`,
      value: i
    }))
  });

  const vault = userVaults[index];
  const now = Math.floor(Date.now() / 1000);

  if (now < vault.timestamp + vault.lockPeriod) {
    console.log(chalk.red('Vault is still locked.'));
    return;
  }

  vault.withdrawn = true;
  console.log(chalk.green(`Withdrawal of ${vault.amount} ${vault.token} successful.`));
}

async function handleEmergency(address) {
  const userVaults = getUserVaults(address).filter(v => !v.withdrawn);

  if (userVaults.length === 0) {
    console.log(chalk.red('No vaults available.'));
    return;
  }

  const { index } = await inquirer.prompt({
    name: 'index',
    type: 'list',
    message: 'Select a vault for emergency withdrawal (10% penalty):',
    choices: userVaults.map((v, i) => ({
      name: `${v.amount} ${v.token} (Locked for ${v.lockPeriod}s)`,
      value: i
    }))
  });

  const vault = userVaults[index];

  const { confirm } = await inquirer.prompt({
    name: 'confirm',
    type: 'confirm',
    message: `Are you sure you want to emergency withdraw ${vault.amount} ${vault.token}? This will incur a 10% fee.`,
  });

  if (!confirm) {
    console.log(chalk.red('Emergency withdrawal cancelled.'));
    return;
  }

  vault.withdrawn = true;
  const amountAfterPenalty = vault.amount * 0.9;
  console.log(chalk.yellow(`Emergency withdrawal complete. You received ${amountAfterPenalty} ${vault.token}`));
}

function displayVaults(userVaults) {
  console.clear();
  if (userVaults.length === 0) {
    console.log(chalk.red('No vaults found.'));
    return;
  }

  console.log(chalk.blue.bold('\nYour Vaults:\n'));
  userVaults.forEach((v, i) => {
    const unlockTime = v.timestamp + v.lockPeriod;
    const now = Math.floor(Date.now() / 1000);
    const isUnlocked = now >= unlockTime;
    console.log(
      `${i + 1}. ${chalk.yellow(v.amount + ' ' + v.token)} | Lock Time: ${v.lockPeriod}s | ${isUnlocked ? chalk.green('Unlocked') : chalk.red('Locked')} | ${v.withdrawn ? chalk.gray('Withdrawn') : chalk.cyan('Active')}`
    );
  });
}

mainMenu();
