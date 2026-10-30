//Normally I'd use rand

// use rand::prelude::*;

use std::io;
use std::io::Write;

#[derive(Debug)]
struct User
{
    id: u32,
    name: String,
    age: u8,
    acc_details: Bank,
}

#[derive(Debug)]
struct Bank
{
    id: u32,
    active: bool,
    logged_in: bool,
    acc_no: u64,
    pin: u32,
    balance: u32,
}

impl Bank
{
    fn make_account(id: u32, active: bool, acc_no: u64, pin: u32, balance: u32) -> Self
    {
        Self 
        {
            id,
            active,
            logged_in: true,
            acc_no,
            pin,
            balance
        }
    }
    
    fn login(accounts: &mut Vec<User>) -> Option<usize>
    {
        print!("Enter your account number: ");
        
        io::stdout()
                    .flush()
                    .expect("Failure to flush data");
        
        let acc_no: u64 = loop
        {
            let mut acc_no: String = String::new();
            
            io::stdin()
                        .read_line(&mut acc_no)
                        .expect("Failure to read account number!");
            
            let acc_no: u64 = match acc_no.trim().parse::<u64>()
            {
                Ok(num) => break num,
                Err(_) => continue,
            };
        };
        
        println!();
        print!("Enter your pin: ");
        println!();
        
        let pin: u32 = loop
        {
            let mut pin: String = String::new();
            
            io::stdin()
                        .read_line(&mut pin)
                        .expect("Failure to read pin!");
            
            let pin: u32 = match pin.trim().parse::<u32>()
            {
                Ok(num) => break num,
                Err(_) => continue,
            };
        };
        
        let mut index_value: usize = 0;
        
        for value in accounts
        {
            if (acc_no == value.acc_details.acc_no)
            {
                println!();
                println!("Account Found!");
                println!();
                
                if (pin == value.acc_details.pin)
                {
                    print!("Welcome {}!", value.name);
                    value.acc_details.logged_in = true;
                    return Some(index_value);
                }
            }
            
            index_value = index_value + 1;
        }
        
        return None;
    }
    
    fn deposit(self: &mut Self)
    {
        if (self.active == false)
        {
            println!();
            println!("Your account has been deactivated. Please activate your account and try again!");
            println!();
            return;
        }
        
        if (self.logged_in == false)
        {
            println!();
            println!("Please Log in and try again!");
            println!();
            return;
        }
        
        println!();
        println!("Deposit Money");
        println!();
        
        let mut deposit: u32 = 0;
        
        print!("Enter the amount you want to deposit: $");
        
        io::stdout()
                    .flush()
                    .expect("Failure to flush out buffer.");
        
        deposit = loop {
            let mut deposit: String = String::new();
            
            io::stdin()
                        .read_line(&mut deposit)
                        .expect("Failure to read deposit value!");
            
            let deposit: u32 = match deposit.trim().parse::<u32>() {
                Ok(num) => break num,
                Err(_) => { println!(); println!("Enter a valid value!"); println!(); continue; },
            };
        };
        
        self.balance = self.balance + deposit;
        
        println!();
        println!("Money deposited successfully!");
        self.check_balance();
    }
    
    fn withdraw(self: &mut Self)
    {
        if (self.active == false)
        {
            println!();
            println!("Your account has been deactivated. Please activate your account and try again!");
            println!();
            return;
        }
        
        if (self.logged_in == false)
        {
            println!();
            println!("Please Log in and try again!");
            println!();
            return;
        }
        
        println!();
        println!("Withdraw Money");
        println!();
        
        let mut withdraw: u32 = 0;
        
        print!("Enter the amount you want to withdraw: $");
        
        io::stdout()
                    .flush()
                    .expect("Failure to flush out buffer.");
        
        withdraw = loop {
            let mut withdraw: String = String::new();
            
            io::stdin()
                        .read_line(&mut withdraw)
                        .expect("Failure to read deposit value!");
            
            let withdraw: u32 = match withdraw.trim().parse::<u32>() {
                Ok(num) => break num,
                Err(_) => { println!(); println!("Enter a valid value!"); println!(); continue; },
            };
        };
        
        if (self.balance < withdraw) {
            println!();
            println!("Cannot withdraw ${withdraw}");
            println!();
            return;
        }
        
        self.balance = self.balance - withdraw;
        
        println!();
        println!("Money withdrawn successfully!");
        self.check_balance();
    }
    
    fn check_balance(self: &Self)
    {
        
        if (self.logged_in == false)
        {
            println!();
            println!("Please Log in and try again!");
            println!();
            return;
        }
        
        println!();
        println!("Current Balance is: ${}", self.balance);
        println!();
    }
    
    fn change_pin(self: &mut Self)
    {
        
        if (self.active == false)
        {
            println!();
            println!("Your account has been deactivated. Please activate your account and try again!");
            println!();
            return;
        }
        
        if (self.logged_in == false)
        {
            println!();
            println!("Please Log in and try again!");
            println!();
            return;
        }
        
        
        
        println!();
        println!("Change Pin working as intended!");
        println!();
    }
    
    fn deactivate_account(self: &mut Self)
    {
        if (self.active == false)
        {
            println!();
            println!("Account is already deactived!");
            self.logged_in = false;
            println!();
            return;
        }
        
        if (self.logged_in == false)
        {
            println!();
            println!("Please Log in and try again!");
            println!();
            return;
        }
        
        println!();
        println!("Are you sure you want to deactivate your account?");
        println!("You will not be able to use the services of the bank going forward");
        println!("And your assets will be frozen until you re-activate your account!");
        println!();
        
        println!("To confirm deactivation, enter Yes");
        println!("To go back, press No");
        
        print!("Enter your choice: ");
        
        io::stdout()
                    .flush()
                    .expect("Failure to flush buffer");
                    
        loop {
            let mut deactivation_choice: String = String::new();
        
            io::stdin()
                        .read_line(&mut deactivation_choice)
                        .expect("Failure to read input");
        
            deactivation_choice.pop();
            deactivation_choice = deactivation_choice.to_lowercase();
        
            if (deactivation_choice == "y" || deactivation_choice == "yes")
            {
                self.active = false;
                self.logged_in = false;
                println!();
                println!("Account has been deactivated successfully!");
                println!();
                break;
            }
            else if (deactivation_choice == "n" || deactivation_choice == "no")
            {
                println!();
                println!("Account deactivation cancelled!");
                println!();
                return;
            }
            else
            {
                println!();
                println!("Please enter a valid choice!");
                println!();
                continue;
            }
        }
        
    }
    
    fn activate_account(self: &mut Self)
    {
    
        if (self.active == true)
        {
            println!();
            println!("Account is already active!");
            println!();
            return;
        }
        
        if (self.logged_in == false)
        {
            println!();
            println!("Please Log in and try again!");
            println!();
            return;
        }
        
        self.active = true;
        
        println!();
        println!("Your rust account has been activated!");
        println!("Enjoy the services!");
        println!();
    }
}

impl User
{
    fn make_user(id: u32, name: String, age: u8, acc_details: Bank) -> Self
    {
        Self
        {
            id,
            name,
            age,
            acc_details,
        }
    }
    
    
}

fn manage_account_utils(user: &mut User) -> u8
{
    
    'bank_account_utils: loop{
        println!();
        
        println!("Choose an action to perform on your account:");
        println!();
    
        println!("1. Change your pin.");
        println!("2. Activate Account.");
        println!("3. Deactivate Account.");
        println!("4. Go Back to Previous Page.");
        println!("5. Logout.");
        println!("6. Logout and Exit.");
        
        println!();
        print!("Enter your choice: ");
        
        io::stdout()
                    .flush()
                    .expect("Failure to flush out buffer.");
                        
        let mut banking_utilities: u8 = loop {
            let mut banking_utilities: String = String::new();
        
            io::stdin()
                        .read_line(&mut banking_utilities)
                        .expect("Failure to read input!");
        
            let mut banking_utilities: u8 = match banking_utilities.trim().parse::<u8>() {
                Ok(num) => break num,
                Err(_) => continue,
            };
        };
            
        match banking_utilities {
            1 => user.acc_details.change_pin(),
            2 => user.acc_details.activate_account(),
            3 => { user.acc_details.deactivate_account(); return 5u8;},
            4 => { println!(); return 4u8; },
            5 => { println!(); return 5u8; },
            6 => { return 6u8; }
            _ => {
                println!();
                println!("Enter a valid choice!");
                println!();
                continue 'bank_account_utils;
            },
        }
    }
}

fn main()
{
    let mut accounts: Vec<User> = Vec::new();
    
    let mut id: u32 = 1;
    let mut age: u8 = 19;
    let mut acc_no: u64 = 12663;
    let mut pin: u32 = 3672;
    let mut balance: u32 = 12_000;
    let mut active: bool = true;
    // let mut logged_in: bool = false;
    
    let mut acc_index: usize = 0;
    let mut banking_utilities: u8 = 0;

    print!("Enter the number of accounts that you want to create: ");
    
    io::stdout()
                .flush()
                .expect("Some error occurred");
                
    let mut n_acc: u8 = loop 
    {
        let mut n_acc: String = String::new();
        
        io::stdin()
                    .read_line(&mut n_acc)
                    .expect("Failure to read value");
        
        let n_acc: u8 = match n_acc.trim().parse::<u8>() 
        {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };
    
    println!();
    println!("The number of accounts to be generated: {n_acc}");
    println!();
    
    
    while (n_acc != 0)
    {
    
        // Random generated values
    /*
        let id: u32 = rand::rng().random_range(1..=100);
        let age: u8 = rand::rng().random_range(16..=65);
        let acc_no: u64 = rand::rng().random_range(1000000..=9999999999999);
        let pin: u32 = rand::rng().random_range(100000..=9999999);
        let balance: u32 = rand::rng().random_range(14500..=999999);
    */
    
        let bank: Bank = Bank::make_account(id, active, acc_no, pin, balance);
        let user: User = User::make_user(id, String::from("Jane Doe"), age, bank);
        
        accounts.push(user);
        
        n_acc = n_acc - 1;
        
        id = id + 5;
        age = age + 5;
        acc_no = acc_no + 737;
        pin = pin + 3637;
        balance= balance + 738;
    }
    
    println!("All accounts have been generated successfully!");
    println!();
    
    'account_login: loop {
    
        acc_index = loop {
            match Bank::login(&mut accounts) {
                None => { 
                    println!("Failure to Authenticate Accound!");
                    println!("Incorrect Account Number or Pin!");
                    println!();
                    continue;
                },
                Some(num) => break num,
            }
        };
    
        println!();
        println!("Login: {}", &accounts[acc_index].acc_details.logged_in);
        println!("Account found at index: {acc_index}");
    
        let user: &mut User = &mut accounts[acc_index];
        
        'bank_utils: loop {
        
            println!("Choose an action to perform on your account:");
            println!();
    
            println!("1. Check Balance.");
            println!("2. Deposit Money.");
            println!("3. Withdraw Money.");
            println!("4. Manage Account.");
            println!("5. Logout.");
            println!("6. Logout and Exit.");
    
            println!();
            print!("Enter your choice: ");
            
            io::stdout()
                        .flush()
                        .expect("Failure to flush buffer.");
        
            banking_utilities = loop {
                let mut banking_utilities: String = String::new();
        
                io::stdin()
                            .read_line(&mut banking_utilities)
                            .expect("Failure to read input!");
        
                let mut banking_utilities: u8 = match banking_utilities.trim().parse::<u8>() {
                    Ok(num) => break num,
                    Err(_) => continue,
                };
            };
        
            match banking_utilities {
                1 => user.acc_details.check_balance(),
                2 => user.acc_details.deposit(),
                3 => user.acc_details.withdraw(),
                4 => match manage_account_utils(user) { // could move this fn into impl User 
                    4 => continue 'bank_utils,
                    5 => continue 'account_login,
                    6 => { return; }
                    _ => ()
                },
                5 => { println!(); continue 'account_login; },
                6 => { return; }
                _ => {
                        println!();
                        println!("Enter a valid choice!");
                        println!();
                        continue 'bank_utils;
                    },
            }
        }
    
    }
        
}
