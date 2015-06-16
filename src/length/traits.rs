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

impl Div for Length {
    type Output = Length;
    
    fn div(self, rhs: Length) -> Length {
        Length::from_meters(self.meters / rhs.meters)
    }
}

impl Mul for Length {
    type Output = Length;
    
    fn mul(self, rhs: Length) -> Length {
        Length::from_meters(self.meters * rhs.meters)
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
