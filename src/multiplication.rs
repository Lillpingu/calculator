use crate::traits::Operation;
pub struct Multiplication;
impl Multiplication {
    pub fn new() -> Self {
        Multiplication
    }
}
impl Operation for Multiplication {
    fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        let multiply = a * b;
        Ok(multiply)
    }
    fn description(&self) -> () {
        println!("Multiplication operation");
    }
}