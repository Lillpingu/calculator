use crate::traits::{CalcValue, Operation};

pub struct Division;
impl Division {
    pub fn new() -> Self {
        Division
    }
}
impl Operation for Division {
    fn calculate(&self, a: CalcValue, b: CalcValue) -> Result<CalcValue, String> {
        Ok(a.div(&b)?)
    }
    fn description(&self) -> () {
        println!("Division operation");
    }
}
