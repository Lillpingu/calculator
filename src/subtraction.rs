use crate::traits::Operation;

pub struct Subtraction;
impl Subtraction {
    pub fn new() -> Self {
        Subtraction
    }
}
impl Operation for Subtraction {
    fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        let subtract = a - b;
        Ok(subtract)
    }
    fn description(&self) -> () {
        println!("Subtraction operation");
    }
}