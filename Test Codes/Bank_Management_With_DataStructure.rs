struct User {
    active: bool,
    uid: u8,
    name: String,
    acc_no: u16,
    pin: u16,
    balance: u16,
}

fn open_account(uid: u8) -> User {
    
    /* let mut name: String = String::new();
    let mut acc_no: String = String::new();
    let mut pin: String = String::new();
    let mut balance: String = Steing::new(); */
    
    User {
        active: true,
        uid,
        name: String::from("John Doe"),
        acc_no: 123344,
        pin: 5677,
        balance: 12_000,
    }
    
    
} 

fn main() {
    println!("Welcome to the Rust Bank!");
    
    let n_acc: u32 = 'number_of_accounts: loop {
    
        println!("Enter the number of accounts to create: ");
    
        let mut n_acc = String::new();
    
        std::io::stdin()
                        .read_line(&mut n_acc)
                        .expect("Failed to read input.");
    
        let n_acc: u32 = match n_acc.trim().parse::<u32>() {
            Ok(num) => break 'number_of_accounts num,
            Err(_) => { println!("Enter a valid number!"); continue 'number_of_accounts; },
        };
        
    };
    
    // println!("Number of Accounts: {n_acc}");
    
    const N_ACC: u32 = n_acc;
    
    let accounts_array: [User; N_ACC];
    
    while n_acc > 0 {
        let i: u8 = 0;
        
        accounts_array[i] = open_account(i);
        
        i = i + 1;
        n_acc = n_acc - 1;
    }
    
    for data in accounts_array {
        println!("Account Id: {}", data.uid);
    }
}
