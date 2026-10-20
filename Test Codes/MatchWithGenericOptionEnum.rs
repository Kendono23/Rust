fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => x.unwrap_or(0i32),
        Some(..) => x.unwrap(),
    }
}


fn main() {
    let x: Option<u32> = None;
    let y: u32 = x.unwrap_or(0);
    println!("{y}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{six}");
    println!("{none}");

}
