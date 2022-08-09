use std::io;

fn main() {
    start_menu();   
}

fn start_menu() {
    'menu: loop {
        println!("-----------------------------------------------");
        println!("Select an option");
        println!("1. Convert degrees Celsius to Fahrenheit");
        println!("2. Convert Fahrenheit to degrees Celsius");
        println!("3. Exit program");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Unable to read the option");

        let option: u8 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("Invalid option");
                continue;
            },
        };
        
        match option {
            1 => {
                let celsius = get_celsius();
                println!("{celsius}° C = {} F", to_fahrenheit(celsius));
                wait_key();
            },
            2 => {
                let fahrenheit = get_fahrenheit();
                println!("{fahrenheit} F = {}° C", to_celsius(fahrenheit));
                wait_key();
            },
            3 => break 'menu,
            _ => {
                println!("Invalid option");
                continue;
            },
        };
    }
}

fn get_fahrenheit() -> f64 {
    let fahrenheit_number: f64;

    loop {
        println!("-----------------------------------------------");
        println!("Enter Fahrenheit:");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect("Unable to read fahrenheit");

        fahrenheit_number = match fahrenheit.trim().parse() {
            Ok(fahrenheit) => fahrenheit,
            Err(_) => {
                println!("Invalid value");
                continue;
            },
        };
        break;
    }
    fahrenheit_number
}

fn get_celsius() -> f64 {
    let celsius_number: f64;

    loop {
        println!("-----------------------------------------------");
        println!("Enter degrees Celsius:");
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius).expect("Unable to read degrees Celsius");

        celsius_number = match celsius.trim().parse() {
            Ok(celsius) => celsius,
            Err(_) => {
                println!("Invalid value");
                continue;
            },
        };
        break;
    }
    celsius_number
}

fn to_celsius(f: f64) -> f64 {
    return (f - 32.0) / 1.8;
}

fn to_fahrenheit(c: f64) -> f64 {
    return c * 1.8 + 32.0;
}

fn wait_key() {
    println!("Press enter to continue");
    let mut none = String::new();
    io::stdin().read_line(&mut none).expect("Error waiting");
}