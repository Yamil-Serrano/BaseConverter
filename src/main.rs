use std::io;
use std::process::Command;

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}
fn main() {
    loop {
        println!("--------------------\nChoose an option:");
        println!("1. Convert binary to decimal");
        println!("2. Convert decimal to binary");
        println!("3. Convert binary to hexadecimal");
        println!("4. Convert hexadecimal to decimal");
        println!("5. Exit \n--------------------");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("--------------------\nFailed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("--------------------\nEnter a binary number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim();
                let decimal = binary_to_dec(input);
                clear_terminal();
                println!("--------------------\nBinary {} in decimal is: {} \n--------------------", input, decimal);
            }
            "2" => {
                println!("Enter a decimal number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim();
                let binary = decimal_to_bin(input);
                clear_terminal();
                println!("--------------------\nDecimal {} in binary is: {} \n--------------------", input, binary);
            }
            "3" => {
                println!("Enter a binary number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim();
                let hex = binary_to_hex(input);
                clear_terminal();
                println!("--------------------\nBinary {} in hexadecimal is: {} \n--------------------", input, hex);
            }
            "4" => {
                println!("Enter a hexadecimal number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim();
                let decimal = hexadecimal_to_decimal(input);
                clear_terminal();
                println!("--------------------\nHexadecimal {} in decimal is: {} \n--------------------", input, decimal);
            }
            "5" => {
                clear_terminal();
                println!("--------------------\nExiting...");
                break;
            }
            _ => println!("--------------------\nInvalid option, please try again.\n--------------------"),
        }

        println!(); // Line break for readability
    }
}

// Function to convert a binary string to a decimal number
fn binary_to_dec(input: &str) -> i32{
    let mut result = 0;
    let mut exponent = 0;

    for bit in input.chars().rev() { // Iterate from right to left using rev()
        // Convert each character into a number (0 or 1)
        let digit = bit.to_digit(10).expect("Invalid binary character!");
        // Compute the decimal value using power of 2
        result += (digit as i32) * 2_i32.pow(exponent);
        exponent += 1;
    }
    result
}

// Function to convert a decimal number (string) to a binary string
fn decimal_to_bin(input: &str) -> String {
    let number: i32 = input.parse().expect("Error trying to convert the number!");
    
    if number == 0 {
        return String::from("0");
    }

    let mut binary = String::new();
    let mut num = number;

    while num > 0 {
        let remainder = num % 2;  // Get the remainder when divided by 2 (1 for odd, 0 for even)
        binary.push_str(&remainder.to_string()); // Append remainder to the string
        num /= 2;  // Divide the number by 2
    }

    // Reverse the binary string to get the correct representation
    binary.chars().rev().collect()
}

// Function to convert a binary string to a hexadecimal string
fn binary_to_hex(input: &str) -> String {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut hex_string = String::new();
    let mut memory = String::new();

    for (i, bit) in input.chars().rev().enumerate() {
        memory.push(bit);  // Store the current bit in a buffer
        
        // Process every 4 bits (or when reaching the end of the string)
        if (i + 1) % 4 == 0 || i == input.len() - 1 {
            let reversed_memory: String = memory.chars().rev().collect(); // Reverse the buffer string
            let decimal_value = binary_to_dec(&reversed_memory); // Convert the reversed binary to decimal
            
            // Convert decimal to hexadecimal
            if decimal_value >= 10 {
                hex_string.push(letters[(decimal_value - 10) as usize]);  // Convert 10-15 to A-F
            } else {
                hex_string.push_str(&decimal_value.to_string()); // Convert 0-9 to string
            }
            memory.clear();  // Clear buffer for the next block
        }
    }

    let reversed_hex: String = hex_string.chars().rev().collect(); // Reverse the final hex string
    reversed_hex
}

fn hexadecimal_to_decimal(input: &str) -> i32 {
    let mut result = 0;
    let mut exponent = 0;

    for ch in input.chars().rev() {
        let digit = match ch {
            '0'..='9' => ch.to_digit(16).expect("Error converting digit"), // Números (0-9)
            'A'..='F' => ch.to_digit(16).expect("Error converting letter"),
            'a'..='f' => ch.to_digit(16).expect("Error converting letter"), 
            _ => panic!("Invalid Hexadecimal Character!"), // Manejo de error
        };

        result += (digit as i32) * 16_i32.pow(exponent);
        exponent += 1;
    }
    result
}