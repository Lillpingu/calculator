use crate::traits::{CalcValue, Operation};

pub struct Subtraction;
impl Subtraction {
    pub fn new() -> Self {
        Subtraction
    }
}
impl Operation for Subtraction {
    fn calculate(&self, a: CalcValue, b: CalcValue) -> Result<CalcValue, String> {
        let subtract = a.sub(&b);
        Ok(subtract)
    }
    fn description(&self) -> () {
        println!("Subtraction operation");
    }
}