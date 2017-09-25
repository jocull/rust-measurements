//! Types and constants for handling angles

use super::measurement::*;
use ::std::f64::consts::PI;

/// The 'Angle' struct can be used to deal with angles in a common way.
///
/// # Example
///
/// ```
/// use measurements::Angle;
///
/// let whole_cake = Angle::from_degrees(360.0);
/// let pieces = 6.0;
/// let slice = whole_cake / pieces;
/// println!("Each slice will be {} degrees", slice.as_degrees());
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Angle {
	radians: f64,
}

/// Number of degrees in a radian
pub const RADIAN_DEGREE_FACTOR: f64 = 180.0 / PI;

impl Angle {

	/// Create a new Angle from a floating point value in degrees
	pub fn from_degrees(degrees: f64) -> Self {
		Angle::from_radians(degrees / RADIAN_DEGREE_FACTOR)
	}

	/// Create a new Angle from a floating point value in radians
	pub fn from_radians(radians: f64) -> Self {
		Angle { radians: radians }
	}

	/// Convert this Angle to a floating point value in degrees
	pub fn as_degrees(&self) -> f64 {
		self.radians * RADIAN_DEGREE_FACTOR
	}

	/// Convert this Angle to a floating point value in radians
	pub fn as_radians(&self) -> f64 {
		self.radians
	}
}

impl Measurement for Angle {
    fn as_base_units(&self) -> f64 {
        self.radians
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_radians(units)
    }

    fn as_base_units_name(&self) -> &'static str {
        "rad"
    }
}

implement_measurement! { Angle }

#[cfg(test)]
mod test {
	use angle::*;
	use test_utils::assert_almost_eq;

	#[test]
	fn radians() {
		let i1 = Angle::from_degrees(360.0);
		let r1 = i1.as_radians();
		let i2 = Angle::from_radians(PI);
		let r2 = i2.as_degrees();
		assert_almost_eq(r1, 2.0 * PI);
		assert_almost_eq(r2, 180.0);
	}
}
