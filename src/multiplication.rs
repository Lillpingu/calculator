use crate::traits::{CalcValue, Operation};
pub struct Multiplication;
impl Multiplication {
    pub fn new() -> Self {
        Multiplication
    }
}
impl Operation for Multiplication {
    fn calculate(&self, a: CalcValue, b: CalcValue) -> Result<CalcValue, String> {
        
        let multiply = a.mul(&b);
        Ok(multiply)
    }
    fn description(&self) -> () {
        println!("Multiplication operation");
    }
}
