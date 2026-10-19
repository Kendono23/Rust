
#[derive(Debug)]
struct Bank
{
    name: String,
    acc_no: u16,
    pin: u16,
    balance: u16,
    login: bool,
}

impl Bank
{
    fn make_user(name: String, acc_no: u16, pin: u16, balance: u16) -> Self
    {
        Self {
            name,
            acc_no,
            pin,
            balance,
            login: false,
        }
    }
    
    fn login(self: &mut Self)
    {
        let mut pin: String = String::new();
        println!("Enter your pin to Login: ");
        
        std::io::stdin().read_line(&mut pin).expect("Please enter a valid pin");
        println!();
        println!("Authenticating pin");
        println!();
        
        let pin: u16 = pin.trim().parse::<u16>().expect("Failed to convert to integer");
        
        if (pin == self.pin)
        {
            println!("Authentication Succes!");
            println!();
            self.login = true;
        }
        else
        {
            println!("Authentication Failure!");
            println!();
            self.login = false;
        }
    }
    
}

impl Bank
{
    
    fn print_details(self: &Self)
    {
        println!();
        println!("Your Account Details");
        println!();
        
        println!("Name: {}", self.name);
        println!("Account Number: {}", self.acc_no);
        println!("Balance: ${}", self.balance);
        
        println!();
    }
    
    fn check_balance(self: &mut Self, login: bool)
    {
        if (login == false)
        {
            self.login();
        }
        
        println!();
        println!("Current Balance: ${}", self.balance);
        println!();
    }
    
    fn deposit(self: &mut Self, amount: u16, login: bool)
    {
        if (login == false)
        {
            self.login();
        }
        
        self.balance = self.balance + amount;
        
        println!();
        println!("Amount ${amount} Deposited!");
        self.check_balance(self.login);
    }
    
    fn withdraw(self: &mut Self, amount: u16, login: bool)
    {
        if (login == false)
        {
            self.login();
        }
        
        self.balance = self.balance - amount;
        
        println!();
        println!("Amount ${amount} Withdrawn!");
        self.check_balance(self.login);
    }  
    
}

fn main()
{
    println!("Welcome to the Rust Bank!");
    let mut u1 = Bank::make_user(String::from("John Doe"), 12344u16, 4532u16, 12_000u16);
    println!();
    println!("Your Account Details");
    println!();
    
    println!("{u1:#?}");
    println!();
    
    u1.login();
    
    u1.check_balance(u1.login);
    u1.print_details();
    u1.deposit(2000u16, u1.login);
    u1.login = false;
    println!("Oops! Server Timed Out. Please Log in Again!");
    u1.withdraw(5000u16, u1.login);
}
