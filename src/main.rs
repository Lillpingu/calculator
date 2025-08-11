mod traits;
mod addition;
mod subtraction;
mod division;
mod multiplication;

use crate::traits::Operation;
fn main() {
    println!("Hello, world!");
    let add = addition::Addition::new();
    let sub = subtraction::Subtraction::new();
    let division = division::Division::new();
    let multiplication = multiplication::Multiplication::new();


    calculate(add);
    calculate(sub);
    calculate(division);
    calculate(multiplication);
}

fn calculate<O:Operation>(operation: O) {
    
    let a = 10.0;
    let b = 5.0;
    operation.description();
    match operation.calculate(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match operation.calculate(a, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
