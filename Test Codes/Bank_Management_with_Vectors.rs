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
    logged_in: bool,
    acc_details: Bank,
}

#[derive(Debug)]
struct Bank
{
    id: u32,
    active: bool,
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
                    value.logged_in = true;
                    return Some(index_value);
                }
            }
            
            index_value = index_value + 1;
        }
        
        return None;
    }
    
    fn deposit(self: &mut Self) -> bool
    {
        todo!();
    }
    
    fn withdraw(self: &mut Self) -> bool
    {
        todo!();
    }
    
    fn check_balance(self: &mut Self)
    {
        todo!();
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
            logged_in: false,
            acc_details,
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
    
    acc_index = loop {
        match Bank::login(&mut accounts) {
            None => { 
                println!("Failure to Authenticate Accound!");
                println!("Incorrect Account Number or Pin!");
                continue;
            },
            Some(num) => break num,
        }
    };
    
    
    println!();
    println!("Login: {}", &accounts[acc_index].logged_in);
    println!("Account found at index: {acc_index}");
    
    
    
        
    }
