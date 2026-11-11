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
        
        phone = 'take_phone: loop {
            
            println!();
            print!("Enter your phone number: ");
            
            io::stdout()
                        .flush()
                        .expect("Failed to flush buffer.");
            
            io::stdin()
                        .read_line(&mut phone)
                        .expect("Failed to read phone number");
            
            phone = phone.pop();
            
            let mut confirm_phone: String = String::new();
            
            println!();
            print!("You have entered: {phone}");
            
            confirm_phone = 'confirm_ph: loop {
                
                println!();
                print!("Confirm phone number? (Y/n): ");
                
                io::stdout()
                            .flush()
                            .expect("Failed to flush buffer.");
                
                io::stdin()
                            .read_line(&mut confirm_phone)
                            .expect("Failed to confirm phone number");
                
                confirm_phone = confirm_phone.to_lowercase().pop();
                
                if (confirm_phone == "y" || confirm_phone == "yes")
                {
                    println!();
                    println!("Phone Number Confirmed!");
                    println!();
                    break 'take_phone phone;
                }
                
                else if (confirm_phone == "n" || confirm_phone == "no")
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
    
    fn book_details(id: u16, book_name: String, book_count: u16) -> Self
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
    let mut number_of_books: u16 = 5;
    
    while (number_of_books > 0) {
        
        let book_data: Book = Book::book_details(number_of_books, String::from("Animal Farm"), 1u16);
        
        book.push(book_data);
        
        number_of_books = number_of_books - 1;
        
    }
    
    let mut id: u32 = 63268;
        
    let library: Library = Library::user_details(id);
        
    let user: User = User::make_user(id, library);
        
    users.push(user);
    
    println!();
    println!("User {id} Generated Successfully!");
    println!();

}

fn login_user(users: &Vec<User>) -> Option<usize> {
    
    let id: u32 = loop {
        
        println!();
        print!("Enter your Library ID: ");
        
        let mut id: String = String::new();
        
        io::stdout()
                    .flush()
                    .expect("Failed to flush buffer");
        
        io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read user id");
        
        let id: u32 = match id.trim().parse::<u32>() {
            Ok(num) => { break num; },
            Err(_) => {
                println!();
                println!("Try again");
                println!();
                continue;
            }
        };
        
    };
    
    let mut phone: String = String::new();
        
        phone = 'take_phone: loop {
            
            println!();
            println!("Phone number: 000-000-0000");
            print!("Enter your phone number: ");
            
            io::stdout()
                        .flush()
                        .expect("Failed to flush buffer.");
            
            io::stdin()
                        .read_line(&mut phone)
                        .expect("Failed to read phone number");
            
            phone = phone.pop();
            
            let mut confirm_phone: String = String::new();
            
            println!();
            print!("You have entered: {phone}");
            
            confirm_phone = 'confirm_ph: loop {
                
                println!();
                print!("Confirm phone number? (Y/n): ");
                
                io::stdout()
                            .flush()
                            .expect("Failed to flush buffer.");
                
                io::stdin()
                            .read_line(&mut confirm_phone)
                            .expect("Failed to confirm phone number");
                
                confirm_phone = confirm_phone.to_lowercase().pop();
                
                if (confirm_phone == "y" || confirm_phone == "yes")
                {
                    println!();
                    println!("Phone Number Confirmed!");
                    println!();
                    break 'take_phone phone;
                }
                
                else if (confirm_phone == "n" || confirm_phone == "no")
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
        
        let mut index: usize = 0;
        
    for user in users {
        if (user.id == id && user.phone == phone)
        {
            println!("Welcome User!");
            return Some(index);
        }
        
        index = index + 1;
    }
    
    return None;
    
}

fn main() {
    
    let mut book: Vec<Book> = Vec::new();
    let mut users: Vec<User> = Vec::new();
    
    let mut number_of_books_in_library: u32 = 0u32;
    
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
    
    number_of_books_in_library = 5 * number_of_users;
    
/*
    
    number_of_books_in_library = loop {
        
        let mut number_of_books_in_library: String = String::new();
        
        println!();
        print!("Enter total number of books in library: ");
        
        io::stdout()
                    .flush()
                    .expect("Failed to flush buffer.");
        
        io::stdin()
                    .read_line(&mut number_of_books_in_library)
                    .expect("Failed to read total number of books");
        
        let number_of_books_in_library: u32 = match number_of_books_in_library.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => continue,
        };
        
    };
    
*/
    
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
    
    let mut active_user: &mut User = loop {
        
        match login_user(&users) {
            Some(index) => { break &mut users[index]; }
            None => {
                println!();
                println!("Incorrect ID or phone number! Please Try Again!");
                println!();
                continue;
            }
        }
        
    };
    
    'library: loop {
    
        let mut choice: u8 = 0;
        
        println!();
        println!("Welcome to Rust Library");
        println!();
        
        println!("What would you like to do today?");
        
        println!("1. Rent a book.");
        println!("2. Exit.");
        println!();
        
        choice = loop {
            
            print!("Enter your choice: ");
            
            let mut choice: String = String::new();
            
            io::stdout()
                        .flush()
                        .expect("Failed to flush buffer");
            
            io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read choice of the user");
            
            let choice: u8 = match choice.trim().parse::<u8>() {
                Ok(num) => { break num; },
                Err(_) => {
                    println!();
                    println!("Please try again!");
                    println!();
                    continue;
                },
            };
            
        };
        
        match choice {
            
            2 => { break 'library; },
            _ => (),
            
        }
        
    }
    
    println!();
    println!("Wish you have a good day!");
    println!();
}
