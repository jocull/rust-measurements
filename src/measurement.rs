pub trait Measurement {
    fn get_base_units(&self) -> f64;
    fn from_base_units(units: f64) -> Self;
}

#[macro_export]
macro_rules! implement_measurement {
    ($($t:ty)*) => ($(
        // TODO: Find a better way to reference these...
        use std::ops::{Add,Sub,Div,Mul};
        use std::cmp::{Eq, PartialEq};
        use std::cmp::{PartialOrd, Ordering};

        impl Add for $t {
            type Output = Self;
            
            fn add(self, rhs: Self) -> Self {
                Self::from_base_units(self.get_base_units() + rhs.get_base_units())
            }
        }

        impl Sub for $t {
            type Output = Self;
            
            fn sub(self, rhs: Self) -> Self {
                Self::from_base_units(self.get_base_units() - rhs.get_base_units())
            }
        }

        /// 
        /// Dividing a `$t` by another `$` returns a ratio.
        /// 
        impl Div<$t> for $t {
            type Output = f64;
            
            fn div(self, rhs: Self) -> f64 {
                self.get_base_units() / rhs.get_base_units()
            }
        }

        /// 
        /// Dividing a `$` by a factor returns a new portion of the measurement.
        /// 
        impl Div<f64> for $t {
            type Output = Self;
            
            fn div(self, rhs: f64) -> Self {
                Self::from_base_units(self.get_base_units() / rhs)
            }
        }

        /// 
        /// Multiplying a `$t` by another `$t` returns the product of those measurements.
        /// 
        impl Mul<$t> for $t {
            type Output = Self;
            
            fn mul(self, rhs: Self) -> Self {
                Self::from_base_units(self.get_base_units() * rhs.get_base_units())
            }
        }

        /// 
        /// Multiplying a `$t` by a factor increases (or decreases) that measurement a number of times.
        /// 
        impl Mul<f64> for $t {
            type Output = Self;
            
            fn mul(self, rhs: f64) -> Self {
                Self::from_base_units(self.get_base_units() * rhs)
            }
        }

        impl Eq for $t { }
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.get_base_units() == other.get_base_units()
            }
        }

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.get_base_units().partial_cmp(&other.get_base_units())
            }
        }
    )*)
}
