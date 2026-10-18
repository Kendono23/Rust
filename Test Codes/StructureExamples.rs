struct User {
    active: bool,
    signincount: u8,
    username: String,
    email: String
}

struct Color (i32, i32, i32);

struct AlwaysEqual;

fn make_user(email: String, username: String) -> User {

    User {
        active: true,
        signincount: 1,
        username,
        email,
    }
}

fn main() {

    let mut email: String = String::new();
    let mut username: String = String::new();
    
    //println!("");
    println!("Enter Email: ");
    std::io::stdin().read_line(&mut email).expect("Failed to read input");
    email.pop();

    println!("");

    println!("Enter Username: ");
    std::io::stdin().read_line(&mut username).expect("Failed to read input");
    username.pop();

    println!("User Data Read successfully");
    
    let mut user1 = make_user(email, username);
    println!("User Data");
    println!("User active: {}", user1.active);
    println!("User sign in count: {}", user1.signincount);
    println!("User email: {}", user1.email);
    println!("Username: {}", user1.username);
    
    println!("Change User email:");
    user1.email.clear();
    let email1= &mut user1.email;
    std::io::stdin().read_line(email1).expect("Failed to read Input");
    email1.pop();
    println!("Updated Email: {}", user1.email);
    
    println!("");
    println!("Color Struct Values");
    
    let Some_Color: Color = Color(123, 234, 111);
    println!("First: {}", Some_Color.0);
    println!("Second: {}", Some_Color.1);
    println!("Third: {}", Some_Color.2);
    
    println!("");
    
    println!("After Destructuring");
    
    let Color(r, g, b) = Some_Color;
    println!("R: {r}");
    println!("G: {g}");
    println!("B: {b}");
    
    
    let always_equal = AlwaysEqual;
    println!("Always Equal Struct Initialized!");
    
}
