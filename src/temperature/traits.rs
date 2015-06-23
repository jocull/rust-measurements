use std::ops::{Add,Sub,Div,Mul};
use std::cmp::{Eq, PartialEq};
use std::cmp::{PartialOrd, Ordering};
use super::Temperature;

impl Add for Temperature {
    type Output = Temperature;
    
    fn add(self, rhs: Temperature) -> Temperature {
        Temperature::from_kelvin(self.kelvin + rhs.kelvin)
    }
}

impl Sub for Temperature {
    type Output = Temperature;
    
    fn sub(self, rhs: Temperature) -> Temperature {
        Temperature::from_kelvin(self.kelvin - rhs.kelvin)
    }
}

impl Div for Temperature {
    type Output = Temperature;
    
    fn div(self, rhs: Temperature) -> Temperature {
        Temperature::from_kelvin(self.kelvin / rhs.kelvin)
    }
}

impl Mul for Temperature {
    type Output = Temperature;
    
    fn mul(self, rhs: Temperature) -> Temperature {
        Temperature::from_kelvin(self.kelvin * rhs.kelvin)
    }
}

impl Eq for Temperature { }
impl PartialEq for Temperature {
    fn eq(&self, other: &Temperature) -> bool {
        self.kelvin == other.kelvin
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Temperature) -> Option<Ordering> {
        self.kelvin.partial_cmp(&other.kelvin)
    }
}
