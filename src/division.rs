use crate::traits::Operation;

pub struct Division;
impl Division {
    pub fn new() -> Self {
        Division
    }
}
impl Operation for Division {
    fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("Division by zero is not allowed".to_string());
        }
        Ok(a / b)
    }
    fn description(&self) -> () {
        println!("Division operation");
    }
}
