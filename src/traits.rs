

pub trait Operation {
    fn calculate(&self, a: f64, b: f64) -> Result<f64, String>;
    fn description(&self) -> ();
}