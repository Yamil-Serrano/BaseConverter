# BaseConverter

**BaseConverter** is a simple, yet powerful command-line tool built with **Rust** that allows you to easily convert numbers between **Binary**, **Decimal**, and **Hexadecimal** bases. This tool was developed as part of a topic for my **Computer Architecture** course, where I gained a deeper understanding of number systems and the importance of base conversions.

## Features

- **Binary ↔ Decimal ↔ Hexadecimal conversion**: Quickly convert between the three most common number systems.
- **Accurate and Reliable**: Uses well-structured logic to ensure precise conversions.
- **Efficient and Fast**: Written in Rust, known for its performance and reliability.
- **Easy to Use**: Input a number in one base and get the corresponding value in another.
- **Expandable**: Room for future enhancements and additional base conversions.

## Installation

To run this app locally, you'll need to have **Rust** installed on your machine. If you haven't installed Rust yet, follow these steps:

1. Go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. Follow the instructions for your operating system.
3. Once Rust is installed, you can clone the repository and run the project using Cargo, Rust's package manager.

```bash
git clone https://github.com/Yamil-Serrano/BaseConverter
cd BaseConverter
cargo run
```

## Usage

You can run the program and provide a decimal, binary, or hexadecimal input to convert between number systems. The program will process the input and return the corresponding values in other bases.

### Example Usage:

```bash
cargo run
```

You will be prompted to enter a number and specify the base for conversion. The program will then display the equivalent values in other number systems.

## Why Rust?

I chose **Rust** for this project because:

- **Performance**: Rust offers fast execution speeds, which makes it ideal for numerical conversions.
- **Safety**: Rust's memory safety guarantees help avoid common issues like null pointers and data races.
- **Low-Level Control**: Rust allows handling bitwise operations efficiently, which is essential for base conversions.
- **Learning**: This project helped me explore how low-level operations in Rust tie into number systems and hardware representation.

## Future Plans

- Implement more base conversions (such as octal).
- Add a user-friendly command-line interface with better input validation.
- Improve performance further by optimizing number parsing and processing.

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Computer Architecture** course: For providing the foundation of knowledge about number systems and conversions.
- **Rust Programming Language**: For being an amazing tool to work with and enabling me to implement this project efficiently.