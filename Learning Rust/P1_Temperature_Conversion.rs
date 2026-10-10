fn main() {

    println!("Choose the temperature scale:\n1. °C\n2. °F");
    let mut scale = String::new();
    
    std::io::stdin()
                    .read_line(&mut scale)
                    .expect("Failed to read the value");
    
    let scale: u8 = match scale.trim().parse() {
        Ok (num) => num,
        Err(_) => 0,
    };
    
    if scale > 2 {
        println!("Enter a valid scale value!");
        return;
    }
    
    let mut temp = String::new();
    
    println!("Enter a Temperature Value: ");
    
    std::io::stdin()
                    .read_line(&mut temp)
                    .expect("Failed to read value.");
    
    let temp: f32 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => { 
          println!("Failed to parse value"); 
          0.0 
        }
    };
    
    println!("Scale: {}", scale);
    println!("Temperature: {temp}");
    
    if scale == 1 {
        println!("Converting °C to °F");
        let degree_fahrenheit: f32 = (temp * (9.0/5.0)) + 32.0;
        println!("°C: {}\n°F: {}", temp, degree_fahrenheit);
    }
    else {
        println!("Converting °F to °C");
        let degree_celcius: f32 = (temp - 32.0) * (5.0/9.0);
        println!("°F: {temp}\n°C: {degree_celcius}")
    }
    
}
