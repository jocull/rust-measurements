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

/// 
/// Dividing a Temperature by another Temperature returns a ratio.
/// 
impl Div<Temperature> for Temperature {
    type Output = f64;
    
    fn div(self, rhs: Temperature) -> f64 {
        self.kelvin / rhs.kelvin
    }
}

/// 
/// Dividing a `Temperature` by a factor returns a new portion of that temperature.
/// 
impl Div<f64> for Temperature {
    type Output = Temperature;
    
    fn div(self, rhs: f64) -> Temperature {
        Temperature::from_kelvin(self.kelvin / rhs)
    }
}

/// 
/// Multiplying a `Temperature` by another `Temperature` returns the product of those temperatures.
/// 
impl Mul<Temperature> for Temperature {
    type Output = Temperature;
    
    fn mul(self, rhs: Temperature) -> Temperature {
        Temperature::from_kelvin(self.kelvin * rhs.kelvin)
    }
}

/// 
/// Multiplying a `Temperature` by a factor increases (or decreases) that temperature a number of times.
/// 
impl Mul<f64> for Temperature {
    type Output = Temperature;
    
    fn mul(self, rhs: f64) -> Temperature {
        Temperature::from_kelvin(self.kelvin * rhs)
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
