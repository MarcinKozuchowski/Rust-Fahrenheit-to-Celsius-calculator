use std::io;

fn main() {
    println!("This program will calculate between Celsius and Fahrenheit for you");
    println!("\nEnter C for Celsius - > Fahrenheit convertion \n
Enter F for Fahrenheit - > Celsius convertion\n
Enter any other character to close the program");
    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("provide a valid input string");
    
    match choice.to_uppercase().trim() {
        "C" => celsius_to_fahrenheit(),
        "F" => fahrenheit_to_celsius(),
        _ => panic!("Invalid choice, run program again"),
}
    pub fn fahrenheit_to_celsius() {
        println!("Enter the temperature in Fahrenheit degrees");
        
            let mut fahrenheit = String::new();
            io::stdin().read_line(&mut fahrenheit) 
                .expect("Enter the temperature in Fahrenheit degrees"); 
                
            let fahrenheit: f64 = fahrenheit.trim().parse()
                .expect("Provide a valid Fahrenheit value");

            let converted_to_celsius = (f64::from(fahrenheit) - 32.0) / 1.8;

            println!("{} Fahrenheit = {} Celsius", fahrenheit, converted_to_celsius);
        }
        pub fn celsius_to_fahrenheit() {
            println!("Enter the temperature in Celsius degrees.");
            
                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius) 
                    .expect("Enter the temperature in Celsius degrees"); 
                    
                let celsius: f64 = celsius.trim().parse()
                    .expect("Provide a valid Celsius Value");

                let converted_to_fahrenheit = f64::from(celsius) * 1.8 + 32.0;

                println!("{} Celsius = {} Fahrenheit", celsius, converted_to_fahrenheit);
            }    
}


