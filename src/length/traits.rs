use std::ops::{Add,Sub,Div,Mul};
use std::cmp::{Eq, PartialEq};
use std::cmp::{PartialOrd, Ordering};
use super::Length;

impl Add for Length {
    type Output = Length;
    
    fn add(self, rhs: Length) -> Length {
        Length::from_meters(self.meters + rhs.meters)
    }
}

impl Sub for Length {
    type Output = Length;
    
    fn sub(self, rhs: Length) -> Length {
        Length::from_meters(self.meters - rhs.meters)
    }
}

/// 
/// Dividing a `Length` by another `Length` returns a ratio.
/// 
impl Div<Length> for Length {
    type Output = f64;
    
    fn div(self, rhs: Length) -> f64 {
        self.meters / rhs.meters
    }
}

/// 
/// Dividing a `Length` by a factor returns a new portion of that length.
/// 
impl Div<f64> for Length {
    type Output = Length;
    
    fn div(self, rhs: f64) -> Length {
        Length::from_meters(self.meters / rhs)
    }
}

/// 
/// Multiplying a `Length` by another `Length` returns the product of those lengths.
/// 
impl Mul<Length> for Length {
    type Output = Length;
    
    fn mul(self, rhs: Length) -> Length {
        Length::from_meters(self.meters * rhs.meters)
    }
}

/// 
/// Multiplying a `Length` by a factor increases (or decreases) that length a number of times.
/// 
impl Mul<f64> for Length {
    type Output = Length;
    
    fn mul(self, rhs: f64) -> Length {
        Length::from_meters(self.meters * rhs)
    }
}

impl Eq for Length { }
impl PartialEq for Length {
    fn eq(&self, other: &Length) -> bool {
        self.meters == other.meters
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Length) -> Option<Ordering> {
        self.meters.partial_cmp(&other.meters)
    }
}
