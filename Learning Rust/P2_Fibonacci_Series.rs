fn main() {
    // Generate nth Fibonacci numbers
    
    let n1 = 0;
    let n2 = 1;
    
    print!("How many Fibonacci numbers do you want to print? ");
    println!();
    
    let mut n = String::new();
    
    std::io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read value");
    
    print!("{}, {}, ", n1, n2);
    
    let n3 = n1 + n2;
    print!("{}, ", n3);
    
    let mut n: u32 = n.trim().parse().expect("Failed to parse value");
    n = n - 3;
    while n > 0 {
        
        let n1 = n2;
        let n2 = n3;
        
        let n3 = n1 + n2;
        print!("{}, ", n3);
    
        n = n - 1;
    }
}
