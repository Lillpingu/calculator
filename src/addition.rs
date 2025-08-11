use crate::traits::Operation;

pub struct Addition;
impl Addition {
    pub fn new() -> Self {
        Addition
    }
}
impl Operation for Addition {
    fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        let add = a + b;
        Ok(add)
    }
    fn description(&self) -> () {
        println!("Addition operation");
    }
}