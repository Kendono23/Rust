//Use rand to generate data of isbn10 and isbn13 and ids

// use rand::prelude::*;

use std::io;
use std::io::Write;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    phone: String,
    library: Library,
}

#[derive(Debug)]
struct Library {
    id: u32,
    books_rented_by_user: Vec<Book>,
    books_in_library: u32,
    rented_books: u8,
    rent_limit: u8,
    overdue_books: u8,
    overdue_charges: u16,
}

#[derive(Debug)]
struct Book {
    isbn10: String,
    isbn13: String,
    book_name: String,
}

impl User {
    
    fn make_user(id: u32, library: Library) -> Self
    {
        
/*        
        let id: u32 = rand::rng.random_range(99999..=1000000);
        
        let mut name: String = String::new();
        
        name = 'take_username: loop {
            
            println!();
            print!("Enter your full name: ");
            println!();
        
            io::stdout()
                        .flush()
                        .expect("Failed to flush buffer.");
        
            io::stdin()
                        .read_line(&mut name)
                        .expect("Failed to read username.");
            
            println!();
            println!("You entered: {name}");
            
            let mut confirm_username: String = String::new;
            
            'confirm_user: loop {
                
                println!();
                print!("Confirm Name? (Y/n): ");
                
                io::stdout()
                            .flush()
                            .expect("Failed to flush buffer.");
                
                io::stdin()
                            .read_line(&mut confirm_username)
                            .expect("Failed to confirm username.");
            
                confirm_username = confirm_username.to_lowercase();
            
                if (confirm_username == "y" || confirm_username == "yes")
                {
                    println!();
                    println!("Username Confirmed!");
                    println!();
                    break 'take_username;
                }
                
                else if (confirm_username == "n" || confirm_username == "no")
                {
                    continue 'take_username;
                }
                
                else
                {
                    println!();
                    println!("Enter valid choice!");
                    println!();
                    continue 'confirm_user;
                }
                
            }
            
        }
        
        let mut phone: String = String::new();
        
        phone = 'take_phone {
            
            println!();
            print!("Enter your phone number: ");
            
            io::stdout()
                        .flush()
                        .expect("Failed to flush buffer.");
            
            io::stdin()
                        .read_line(&mut phone)
                        .expect("Failed to read phone number");
            
            let mut confirm_phone: String = String::new();
            
            println!();
            print!("You have entered: {phone}");
            
            confirm_phone = 'confirm_ph loop {
                
                println!();
                print!("Confirm phone number? (Y/n): ");
                
                io::stdout()
                            .flush()
                            .expect("Failed to flush buffer.");
                
                io.stdin()
                            .read_line(&mut confirm_phone)
                            .expect("Failed to confirm phone number");
                
                if (confirm_username == "y" || confirm_username == "yes")
                {
                    println!();
                    println!("Phone Number Confirmed!");
                    println!();
                    break 'take_phone;
                }
                
                else if (confirm_username == "n" || confirm_username == "no")
                {
                    continue 'take_phone;
                }
                
                else
                {
                    println!();
                    println!("Enter valid choice!");
                    println!();
                    continue 'confirm_ph;
                }
                
            }
            
        }
        
*/
        
        Self {
            id,
            name: String::from("John Doe"),
            phone: String::from("123-456-7890"),
            library,
        }
        
    }
    
    fn update_user()
    {
        todo!();
    }
}

impl Book {
    
    fn book_details(book_name: String) -> Self
    {
/*
        
        let isbn10: String = rand::rng()
                                    .random_range(1000000000..=9999999999)
                                    .to_string();
        
        let isbn10: String = rand::rng()
                                    .random_range(1000000000000..=9999999999999)
                                    .to_string();
        
        let mut book_name: String = String::new();
        
        println!();
        print!("Enter book name: ");
        
        io::stdout()
                    .flush()
                    .expect("Failed to flush buffer.");
        
        io::stdin()
                    .read_line(&mut book_name)
                    .expect("Failed to read book name");
        
        println!();
        println!("Book added successfully!");
        println!();
        
*/
        
        Self {
            isbn10: String::from("3628284657"),
            isbn13: String::from("362
