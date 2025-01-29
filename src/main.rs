fn main() {
    // Example of binary to decimal conversion:
    let input = "11111010010000101"; 
    let decimal = binary_to_decimal(input);
    println!("The binary number {} in decimal is: {}", input, decimal);

    // Example of decimal to binary conversion:
    let input = "128133"; // Example of decimal number
    let binary = decimal_to_binary(input);
    println!("The decimal number {} in binary is: {}", input, binary);
}

fn binary_to_decimal(input: &str) -> i32{
    let mut result = 0;
    let mut exponent = 0;

    for bit in input.chars().rev() { // iterate from right to left using rev()
        // Convert each character into a number
        let digit = bit.to_digit(10).expect("Invalid binary character!");
        // Calculate the decimal value
        result += (digit as i32) * 2_i32.pow(exponent);
        exponent += 1;
    }
    result
}

fn decimal_to_binary(input: &str) -> String {
    let number: i32 = input.parse().expect("Error trying to convert the number!");
    
    if number == 0 {
        return String::from("0");
    }

    let mut binary = String::new();
    let mut num = number;

    while num > 0 {
        let remainder = num % 2;  // If the number divided by 2 is odd, the remainder will be 1; otherwise, it will be 0
        binary.push_str(&remainder.to_string()); // Add the remainder to the string
        num /= 2;  // Divide the number by 2
    }

    // Rev the binary number to have the correct orientation
    binary.chars().rev().collect()
}