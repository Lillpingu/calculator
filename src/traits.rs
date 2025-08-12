#[derive(Debug)]
pub enum CalcValue {
    Int(i128),
    Float(f64),
}
impl CalcValue {
  pub fn add(&self, other: &CalcValue) -> CalcValue {
        match (self, other) {
            (CalcValue::Int(a), CalcValue::Int(b)) => CalcValue::Int(a + b),
            (CalcValue::Float(a), CalcValue::Float(b)) => CalcValue::Float(a + b),
            (CalcValue::Int(a), CalcValue::Float(b)) => CalcValue::Float(*a as f64 + b),
            (CalcValue::Float(a), CalcValue::Int(b)) => CalcValue::Float(a + *b as f64),
        }
    }
    pub fn sub(&self, other: &CalcValue) -> CalcValue {
        match (self, other) {
            (CalcValue::Int(a), CalcValue::Int(b)) => CalcValue::Int(a - b),
            (CalcValue::Float(a), CalcValue::Float(b)) => CalcValue::Float(a - b),
            (CalcValue::Int(a), CalcValue::Float(b)) => CalcValue::Float(*a as f64 - b),
            (CalcValue::Float(a), CalcValue::Int(b)) => CalcValue::Float(a - *b as f64),
        }
    }
    pub fn mul(&self, other: &CalcValue) -> CalcValue {
        match (self, other) {
            (CalcValue::Int(a), CalcValue::Int(b)) => CalcValue::Int(a * b),
            (CalcValue::Float(a), CalcValue::Float(b)) => CalcValue::Float(a * b),
            (CalcValue::Int(a), CalcValue::Float(b)) => CalcValue::Int(*a * *b as i128),
            (CalcValue::Float(a), CalcValue::Int(b)) => CalcValue::Int(*a as i128 * *b),
        }
    }
    pub fn is_zero(&self) -> bool {
        match self {
            CalcValue::Int(a) => *a == 0,
            CalcValue::Float(a) => *a == 0.0,
        }
    }
    pub fn div(&self, other: &CalcValue) -> Result<CalcValue, String> {
        if other.is_zero() {
            return Err("Division by zero is not allowed".to_string());
        }
        match (self, other) {
            (CalcValue::Int(a), CalcValue::Int(b)) => Ok(CalcValue::Int(a / b)),
            (CalcValue::Float(a), CalcValue::Float(b)) => Ok(CalcValue::Float(a / b)),
            (CalcValue::Int(a), CalcValue::Float(b)) => Ok(CalcValue::Float(*a as f64 / b)),
            (CalcValue::Float(a), CalcValue::Int(b)) => Ok(CalcValue::Float(a / *b as f64)),
        }
    }
  
}

pub trait Operation {
    fn calculate(&self, a: CalcValue, b: CalcValue) -> Result<CalcValue, String>;
    fn description(&self) -> ();
}