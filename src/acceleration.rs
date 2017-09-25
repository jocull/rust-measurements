use super::measurement::*;
use super::*;
use std::time::Duration;

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
    pub fn from_meters_per_second_per_second(meters_per_second_per_second: f64) -> Acceleration {
        Acceleration { meters_per_second_per_second: meters_per_second_per_second }
    }

    pub fn from_metres_per_second_per_second(metres_per_second_per_second: f64) -> Acceleration {
        Acceleration::from_meters_per_second_per_second(metres_per_second_per_second)
    }

    pub fn from_feet_per_second_per_second(feet_per_second_per_second: f64) -> Acceleration {
        Acceleration::from_metres_per_second_per_second(feet_per_second_per_second / length::METER_FEET_FACTOR)
    }

    pub fn as_meters_per_second_per_second(&self) -> f64 {
        self.meters_per_second_per_second
    }

    pub fn as_metres_per_second_per_second(&self) -> f64 {
        self.as_meters_per_second_per_second()
    }

    pub fn as_feet_per_second_per_second(&self) -> f64 {
        self.meters_per_second_per_second * length::METER_FEET_FACTOR
    }
}

/// Acceleration * Time = Speed
impl ::std::ops::Mul<Duration> for Acceleration {
    type Output = Speed;

    fn mul(self, rhs: Duration) -> Speed {
        Speed::from_meters_per_second(self.as_meters_per_second_per_second() * duration_as_f64(rhs))
    }
}

/// Time * Acceleration = Speed
impl ::std::ops::Mul<Acceleration> for Duration {
    type Output = Speed;

    fn mul(self, rhs: Acceleration) -> Speed {
        rhs * self
    }
}

/// mass * acceleration = Force
impl ::std::ops::Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, rhs: Mass) -> Force {
        Force::from_newtons(rhs.as_kilograms() * self.as_meters_per_second_per_second())
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
