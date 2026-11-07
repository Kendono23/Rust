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
    rented_books: u8,
    rent_limit: u8,
    overdue_books: u8,
    overdue_charges: u16,
}

#[derive(Debug)]
struct Book {
    id: u16,
    isbn10: String,
    isbn13: String,
    book_name: String,
    book_count: u16,
    books_in_library: u16,
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
                
            };
            
        };
        
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
                
            };
            
        };
        
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
    
    fn book_details(id: u16, book_name: String, book_count: u16, books_in_library: u16) -> Self
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
        
        let book_count: u16 = loop {
            
            let mut book_count: String = String::new();
            
            println!();
            print!("Enter book count: ");
            
            io::stdout()
                        .flush()
                        .expect("Failed to flush buffer");
            
            io::stdin()
                        .read_line(&mut book_count)
                        .expect("Failed to read book count");
            
            let book_count: u16 = match book_count.trim().parse::<u16>() {
                Ok(num) => break num,
                Err(_) => continue,
            };
            
        };
        
*/
        
        Self {
            id,
            isbn10: String::from("3628284657"),
            isbn13: String::from("3628910373153"),
            book_name,
            book_count,
            books_in_library,
        }
        
    }
    
    fn update_book() {
        todo!();
    }
}

impl Library {
    
    fn user_details(id: u32) -> Self {
        
/*
        
        let id: u32 = rand::rng().random_range(10000..=9999999);
        
*/
        
        let books_rented_by_user: Vec<Book> = Vec::new();
        
        Self {
            id,
            books_rented_by_user,
            rented_books: 0u8,
            rent_limit: 10u8,
            overdue_books: 0u8,
            overdue_charges: 0u16,
        }
        
    }
    
    fn rent_book() {
        todo!();
    }
}

fn generate_data(users: &mut Vec<User>, book: &mut Vec<Book>)
{
    let mut number_of_books: u16 = 5 * 2;
    
    while (number_of_books > 0) {
        
        let book_data: Book = Book::book_details(number_of_books, String::from("Animal Farm"), 1u16, number_of_books);
        
        book.push(book_data);
        
        number_of_books = number_of_books - 1;
        
    }
    
    let mut id: u32 = 63268;
        
    let library: Library = Library::user_details(id);
        
    let user: User = User::make_user(id, library);
        
    users.push(user);
    
    println!();
    println!("Users Generated Successfully!");
    println!();

}

fn main() {
    
    let mut book: Vec<Book> = Vec::new();
    let mut users: Vec<User> = Vec::new();
    
    let mut number_of_users: u32 = loop {
        
        println!();
        print!("Enter the number of users: ");
        
        let mut number_of_users: String = String::new();
        
        io::stdout()
                    .flush()
                    .expect("Failed to flush buffer.");
        
        io::stdin()
                    .read_line(&mut number_of_users)
                    .expect("Failed to read number of users");
        
        let number_of_users: u32 = match number_of_users.trim().parse::<u32>() {
            Ok(num) => { break num; },
            Err(_) => {
                println!();
                println!("Try Again!");
                println!();
                continue;
            },
        };
        
    };
    
    while (number_of_users > 0) {
    
        let mut_user = &mut users;
        let mut_book = &mut book;
    
        generate_data(mut_user, mut_book);
        
        number_of_users = number_of_users - 1;
        
    }
    
    for user in &users {
        
        println!();
        println!("{user:#?}");
        println!();
        
    }
}
