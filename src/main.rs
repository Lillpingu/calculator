mod traits;
mod addition;
mod subtraction;
mod division;
mod multiplication;

use crate::traits::Operation;
use std::io;
fn main() {
    // interactive loop to read user input
    // and perform calculations based on the input
    // input format: "number operator number"
    // where operator can be +, -, *, x, X, ÷, / or :
    // exit the loop by typing "exit"
    loop {
        println!("Enter an expression using two numbers and an operator (+, -, *, x, /) or type 'exit' to quit:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim(); // trim whitespace from the input
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break; // exit the loop to terminate the program
        }
        if input.is_empty() {
            continue; // skip empty input and prompt again
        }
        // parse the input and extract the numbers and operator
        // if parsing fails, print an error message and prompt again
        // if parsing succeeds, perform the calculation
        match parse_input(input) {
            Ok((left_str, op_char, right_str)) => {
                
                // parse the left and right parts of the input as f64
                let a: f64 = match left_str.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid number: {}", left_str);
                        continue;
                    }
                };
                let b: f64 = match right_str.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid number: {}", right_str);
                        continue;
                    }
                };
                match op_char {
                    '+' => {
                        let op = addition::Addition::new();
                        calculate_with_values(op, a, b);
                    }
                    '-' => {
                        let op = subtraction::Subtraction::new();
                        calculate_with_values(op, a, b);
                    }
                    '*' | 'x' | 'X' | '×' => {
                        let op = multiplication::Multiplication::new();
                        calculate_with_values(op, a, b);
                    }
                    '/' | ':' | '÷' => {
                        let op = division::Division::new();
                        calculate_with_values(op, a, b);
                    }
                    _ => println!("Unsupported operator: {}", op_char), // handle unsupported operators
                }
            }
            Err(e) => println!("Error parsing input: {}", e), // print error if parsing fails
        }
    }
}


// Generic calculator wrapper that prints the result or error.
// Takes an operation that implements the `Operation` trait and two numbers.
fn calculate_with_values<O: Operation>(operation: O, a: f64, b: f64) {
    operation.description(); // Print the description of the operation
    match operation.calculate(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn normalize_decimal(s: &str) -> String {
    // Accept "," or "." as decimal; normalize commas to dots for Rust parsing.
    s.trim().replace(',', ".")
}

/// Parse user input of the form \"number operator number\" into its parts.
/// Recognises +, -, *, x, X, ÷, / and :
fn parse_input(input: &str) -> Result<(String, char, String), String> {
    let mut op_index: Option<usize> = None;
    let mut op_char: char = ' ';
    for (i, c) in input.chars().enumerate() {
        // skip a leading minus sign so negative numbers aren’t mistaken for the operator
        if i == 0 && c == '-' {
            continue;
        }
        // check if the character is one of the supported operators
        // if it is, we store its index and character
        // and break the loop to avoid finding multiple operators
        if "+-*/xX:÷×".contains(c) {
            op_index = Some(i); // store the index of the operator
            op_char = c; // store the operator character
            break;
        }
    }
    // if no operator was found, return an error
    let idx = op_index.ok_or_else(|| String::from("No operator found in input"))?;
    //let left = input[..idx].trim().to_string();
    //let right = input[idx + 1..].trim().to_string();
    let left = normalize_decimal(&input[..idx]);
    let right = normalize_decimal(&input[idx + 1..]);
    Ok((left, op_char, right))
}

// Save result to a json file
// access history of calculations