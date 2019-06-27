use std::io;

enum ConversionType {
    Fahrenheit,
    Celsius,
    None,
}

fn main() {
    println!("Convert Fahrenheit to Celsius or vice versa.");
    let conversion_type = ask_for_temperature_option();
    let conversion_amount: i64;
    match conversion_type {
        ConversionType::Celsius | ConversionType::Fahrenheit => {
            conversion_amount = ask_for_amount_input();
        },
        _ => return,
    }

    match conversion_type {
        ConversionType::Celsius => convert_value_to_celsius(conversion_amount),
        ConversionType::Fahrenheit => convert_value_to_farenheit(conversion_amount),
        _ => return,
    }
}

fn ask_for_temperature_option() -> ConversionType {
    let mut first_run = true;

    loop {
        if first_run {
            println!("Please input f for Fahrenheit or c for Celsius as the input to convert or quit (q):");
        } else {
            println!("Please try again. Input f for Fahrenheit or c for Celsius as the input to convert or quit (q):");
        }

        first_run = false;

        let mut input = String::new();

        match io::stdin().read_line(& mut input) {
            Ok(_) => (),
            Err(err) => {
                println!("Error on input: {}", err);
                continue;
            }
        };

        let input = input.trim().to_lowercase();

        match input.as_ref() {
            "f" => return ConversionType::Fahrenheit,
            "c" => return ConversionType::Celsius,
            "quit" | "q" => return ConversionType::None,
            _ => {
                println!("No options for input");
                continue;
            },
        }
    }
}

fn ask_for_amount_input() -> i64 {
    let mut first_run = true;

    loop {
        if first_run {
            println!("Please input conversion amount:");
        } else {
            println!("Please input conversion amount again:");
        }
        first_run = false;

        let mut input = String::new();

        match io::stdin().read_line(& mut input) {
            Ok(_) => (),
            Err(err) => {
                println!("Error on input: {}", err);
                continue;
            }
        };

        let input: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error on input: {}", err);
                continue;
            }
        };

        return input;
    }
}

fn convert_value_to_farenheit(x: i64) {
    println!("{:?}", ((x as f64) - 32.0) * (5.0/9.0));
}

fn convert_value_to_celsius(x: i64) {
    println!("{:?}", ((x as f64) * (9.0/5.0)) + 32.0);
}
