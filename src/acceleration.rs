//! Types and contants for handling acceleration.

use super::measurement::*;
use super::length;

/// The `Acceleration` struct can be used to deal with Accelerations in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::{Acceleration, Length, Speed};
/// use std::time::Duration;
///
/// // Standing quarter mile in 10.0 dead, at 120.0 mph
/// let track = Length::from_miles(0.25);
/// let finish = Speed::from_miles_per_hour(120.0);
/// let time = Duration::new(10, 0);
/// let accel = finish / time;
/// println!("You accelerated over {} at an average of {}", track, accel);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Acceleration {
    meters_per_second_per_second: f64,
}

impl Acceleration {
    /// Create a new Acceleration from a floating point value in meters per second per second
    pub fn from_meters_per_second_per_second(meters_per_second_per_second: f64) -> Acceleration {
        Acceleration { meters_per_second_per_second: meters_per_second_per_second }
    }

    /// Create a new Acceleration from a floating point value in metres per second per second
    pub fn from_metres_per_second_per_second(metres_per_second_per_second: f64) -> Acceleration {
        Acceleration::from_meters_per_second_per_second(metres_per_second_per_second)
    }

    /// Create a new Acceleration from a floating point value in feet per second per second
    pub fn from_feet_per_second_per_second(feet_per_second_per_second: f64) -> Acceleration {
        Acceleration::from_metres_per_second_per_second(
            feet_per_second_per_second / length::METER_FEET_FACTOR,
        )
    }

    /// Convert this Acceleration to a value in meters per second per second
    pub fn as_meters_per_second_per_second(&self) -> f64 {
        self.meters_per_second_per_second
    }

    /// Convert this Acceleration to a value in metres per second per second
    pub fn as_metres_per_second_per_second(&self) -> f64 {
        self.as_meters_per_second_per_second()
    }

    /// Convert this Acceleration to a value in feet per second per second
    pub fn as_feet_per_second_per_second(&self) -> f64 {
        self.meters_per_second_per_second * length::METER_FEET_FACTOR
    }
}

impl Measurement for Acceleration {
    fn get_base_units(&self) -> f64 {
        self.meters_per_second_per_second
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_meters_per_second_per_second(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "m/s\u{00B2}"
    }
}

implement_measurement! { Acceleration }

#[cfg(test)]
mod test {

    use super::*;
    use tests::assert_almost_eq;
    use std::time::Duration;
    use speed::Speed;

    // Metric
    #[test]
    fn speed_over_time() {
        let s1 = Speed::from_meters_per_second(10.0);
        let t1 = Duration::new(5, 0);
        let i1 = s1 / t1;
        let r1 = i1.as_meters_per_second_per_second();
        assert_almost_eq(r1, 2.0);
    }

    // Traits
    #[test]
    fn add() {
        let a = Acceleration::from_meters_per_second_per_second(2.0);
        let b = Acceleration::from_meters_per_second_per_second(4.0);
        let c = a + b;
        let d = b + a;
        assert_almost_eq(c.as_meters_per_second_per_second(), 6.0);
        assert_eq!(c, d);
    }

    #[test]
    fn sub() {
        let a = Acceleration::from_meters_per_second_per_second(2.0);
        let b = Acceleration::from_meters_per_second_per_second(4.0);
        let c = a - b;
        assert_almost_eq(c.as_meters_per_second_per_second(), -2.0);
    }

    #[test]
    fn mul() {
        let a = Acceleration::from_meters_per_second_per_second(3.0);
        let b = a * 2.0;
        let c = 2.0 * a;
        assert_almost_eq(b.as_meters_per_second_per_second(), 6.0);
        assert_eq!(b, c);
    }

    #[test]
    fn div() {
        let a = Acceleration::from_meters_per_second_per_second(2.0);
        let b = Acceleration::from_meters_per_second_per_second(4.0);
        let c = a / b;
        let d = a / 2.0;
        assert_almost_eq(c, 0.5);
        assert_almost_eq(d.as_meters_per_second_per_second(), 1.0);
    }

    #[test]
    fn eq() {
        let a = Acceleration::from_meters_per_second_per_second(2.0);
        let b = Acceleration::from_meters_per_second_per_second(2.0);
        assert_eq!(a == b, true);
    }

    #[test]
    fn neq() {
        let a = Acceleration::from_meters_per_second_per_second(2.0);
        let b = Acceleration::from_meters_per_second_per_second(4.0);
        assert_eq!(a == b, false);
    }

    #[test]
    fn cmp() {
        let a = Acceleration::from_meters_per_second_per_second(2.0);
        let b = Acceleration::from_meters_per_second_per_second(4.0);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
    }

}
