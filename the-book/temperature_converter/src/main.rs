use std::io;

fn main() {
    //temperature_converter();
    celsius_to_fahrenheit();
}

fn celsius_to_fahrenheit() {
    println!("testes");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read user input.");

    let temp = ( temp * 9 / 5 ) - 32;
    println!("the temperature is: {temp}F");
}

fn fahrenheit_to_celsius() {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input.");

    let temp = 0;
    let temp = ( temp * 5 / 9 ) + 32;
    println!("the temperature is: {temp}C");
}

fn temperature_converter() {
    println!("This program will convert the temperature for you.");

    println!("do you want to convert TO Fahrenheit or TO Celsius? enter 'F' or 'C'.");

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input.");

        println!("{user_input}");
    };
}
