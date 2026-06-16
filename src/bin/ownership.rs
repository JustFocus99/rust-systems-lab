/*
Create a small Rust file that models a simple account-like object.
The object must support one operation that can succeed or fail.

Required Behavior
Your program should show:
1. Create one value.✅
2. Read something from it using an immutable reference.✅
3. Modify it using a mutable reference.✅
4. Return success for a valid operation.✅
5. Return error for an invalid operation.✅

Expected Output / Test Examples
Example behavior, not final code:
Initial account:
id = alice
balance = 100

Read balance:
100

Deposit 50:
success

Final balance:
150

Deposit 0:
failure
 */

#[derive(Debug)]
struct Account {
    id: String,
    name: String,
    balance: u64,
}

enum DepositErr {
    ZeroDeposit,
}

impl Account {
    fn deposit(&mut self, amount: u64) -> Result<(), DepositErr> {
        if amount == 0 {
            return Err(DepositErr::ZeroDeposit);
        }

        let balance = &mut self.balance;
        *balance += amount;
        Ok(())
    }
}

fn main() {
    let mut alice = Account {
        id: String::from("alice"),
        name: String::from("alice"),
        balance: 100,
    };

    // immutable reference
    let alice_name = &alice.name;
    let alice_balance = &alice.balance;
    println!("{:?} has balance: {:?}", alice_name, alice_balance);

    // mutable reference
    let mut_name = &mut alice.name;
    *mut_name = String::from("AAlice");
    let mut_balance = &mut alice.balance;
    *mut_balance = 50;
    println!("{:#?}", alice);

    match alice.deposit(0) {
        Ok(()) => println!("Deposit Success!\n{:#?}", alice),
        Err(e) => match e {
            DepositErr::ZeroDeposit => println!("Can't Deposit Zero"),
        },
    }
}
