use crate::traits::{CalcValue, Operation};

pub struct Addition;
impl Addition {
    pub fn new() -> Self {
        Addition
    }
}
impl Operation for Addition {
    fn calculate(&self, a: CalcValue, b: CalcValue) -> Result<CalcValue, String> {
        Ok(a.add(&b))
    }
    fn description(&self) -> () {
        println!("Addition operation");
    }
}