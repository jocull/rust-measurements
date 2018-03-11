//! Types and constants for handling speed of rotation (angular velocity)

use super::measurement::*;
use core::f64::consts::PI;

/// The 'AngularVelocity' struct can be used to deal with angular velocities in a common way.
///
/// # Example
///
/// ```
/// use measurements::AngularVelocity;
///
/// const cylinders: f64 = 6.0;
/// let engine_speed = AngularVelocity::from_rpm(9000.0);
/// let sparks_per_second = (engine_speed.as_hertz() / 2.0) * cylinders;
/// ```
#[derive(Copy, Clone, Debug)]
pub struct AngularVelocity {
    radians_per_second: f64,
}

impl AngularVelocity {
    /// Create a new AngularVelocity from a floating point value in radians per second
    pub fn from_radians_per_second(radians_per_second: f64) -> Self {
        AngularVelocity { radians_per_second: radians_per_second }
    }

    /// Create a new AngularVelocity from a floating point value in revolutions per minute (RPM)
    pub fn from_rpm(rpm: f64) -> Self {
        let revs_per_second = rpm / 60.0;
        AngularVelocity::from_radians_per_second(revs_per_second * PI * 2.0)
    }

    /// Create a new AngularVelocity from a floating point value in revolutions per second (Hz)
    pub fn from_hertz(hz: f64) -> Self {
        AngularVelocity::from_radians_per_second(hz * PI * 2.0)
    }

    /// Convert this AngularVelocity to a floating point value in radians per second
    pub fn as_radians_per_second(&self) -> f64 {
        self.radians_per_second
    }

    /// Convert this AngularVelocity to a floating point value in degrees
    pub fn as_rpm(&self) -> f64 {
        (self.radians_per_second * 60.0) / (2.0 * PI)
    }

    /// Convert this AngularVelocity to a floating point value in revolutions per second (Hz)
    pub fn as_hertz(&self) -> f64 {
        self.radians_per_second / (2.0 * PI)
    }
}

impl Measurement for AngularVelocity {
    fn as_base_units(&self) -> f64 {
        self.radians_per_second
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_radians_per_second(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "rad/s"
    }
}

implement_measurement! { AngularVelocity }

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::assert_almost_eq;

    #[test]
    fn rpm() {
        let i1 = AngularVelocity::from_rpm(6000.0);
        let r1 = i1.as_radians_per_second();
        let i2 = AngularVelocity::from_radians_per_second(100.0);
        let r2 = i2.as_rpm();
        assert_almost_eq(r1, 628.31853);
        assert_almost_eq(r2, 954.929659642538);
    }
}
