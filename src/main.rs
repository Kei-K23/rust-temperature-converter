use std::io;

#[derive(PartialEq)]
enum TempType {
    F,
    C,
}

fn main() {
    let mut input_temp = String::new();

    println!("Enter temperature value:");

    // Reads a line of input from the user.
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read temperature");

    // Parses a string into a floating-point number.
    let input_temp: f32 = input_temp
        .trim()
        .parse()
        .expect("Failed to parse temperature");

    let mut input_temp_type = String::new();

    println!("What do you want to convert celsius for type c/C and fahrenheit for type f/F:");

    // Reads a line of input from the user.
    io::stdin()
        .read_line(&mut input_temp_type)
        .expect("Failed to read temperature type");

    // Compares two strings for equality, ignoring case.
    let input_temp_type = match input_temp_type.trim().to_uppercase().as_str() {
        "C" => TempType::C,
        "F" => TempType::F,
        _ => panic!("Invalid temperature type"),
    };

    // Returns the value of one of the two arguments if the condition is true, or the other argument if the condition is false.
    let initial_temp_type = if TempType::C == input_temp_type {
        TempType::F
    } else {
        TempType::C
    };

    // Returns a string converted to lowercase.
    let convert_temp_type = match input_temp_type {
        TempType::C => "Celsius",
        TempType::F => "Fahrenheit",
    }
    .to_lowercase();

    // Returns a string converted to lowercase.
    let convert_initial_temp_type = match initial_temp_type {
        TempType::C => "Fahrenheit",
        TempType::F => "Celsius",
    }
    .to_lowercase();

    // Calculates the temperature conversion.
    let converted_temp = convert_temp(input_temp, input_temp_type);

    // Prints a formatted string to the standard output.
    println!("Converted temperature from {input_temp} {convert_initial_temp_type} is {converted_temp} {convert_temp_type}");
}

// calculate the temperature
fn convert_temp(temp: f32, temp_type: TempType) -> f32 {
    match temp_type {
        TempType::F => temp * 9.0 / 5.0 + 32.0,
        TempType::C => (temp - 32.0) * 5.0 / 9.0,
    }
}
