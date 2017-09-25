//! Types and contants for handling speed.

use super::measurement::*;
use super::*;

/// Number of seconds in a minute
pub const SECONDS_MINUTES_FACTOR: f64 = 60.0;
/// Number of minutes in a hour
pub const MINUTES_HOURS_FACTOR: f64 = 60.0;
/// Number of seconds in a hour
pub const SECONDS_HOURS_FACTOR: f64 = 60.0 * 60.0;

/// The `Speed` struct can be used to deal with speeds in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::Speed;
///
/// let light = Speed::from_meters_per_second(300_000_000.0);
/// let mph = light.as_miles_per_hour();
/// println!("The speed of light is {} mph.", mph);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Speed {
    meters_per_second: f64,
}

impl Speed {
    /// Create a new Speed from a floating point number of m/s
    pub fn from_meters_per_second(meters_per_second: f64) -> Speed {
        Speed { meters_per_second: meters_per_second }
    }

    /// Create a new Speed from a floating point number of m/s
    pub fn from_metres_per_second(metres_per_second: f64) -> Speed {
        Speed::from_meters_per_second(metres_per_second)
    }

    /// Create a new Speed from a floating point number of km/h (kph)
    pub fn from_kilometers_per_hour(kilometers_per_hour: f64) -> Speed {
        Speed::from_meters_per_second(
            (kilometers_per_hour / length::METER_KILOMETER_FACTOR) / SECONDS_HOURS_FACTOR)
    }

    /// Create a new Speed from a floating point number of km/h (kph)
    pub fn from_kilometres_per_hour(kilometres_per_hour: f64) -> Speed {
        Speed::from_kilometers_per_hour(kilometres_per_hour)
    }

    /// Create a new Speed from a floating point number of miles/hour (mph)
    pub fn from_miles_per_hour(miles_per_hour: f64) -> Speed {
        Speed::from_meters_per_second((miles_per_hour * 1609.0) / 3600.0)
    }

    /// Convert this speed to a floating point number of m/s
    pub fn as_meters_per_second(&self) -> f64 {
        self.meters_per_second
    }

    /// Convert this speed to a floating point number of m/s
    pub fn as_metres_per_second(&self) -> f64 {
        self.as_meters_per_second()
    }

    /// Convert this speed to a floating point number of km/hour (kph)
    pub fn as_kilometers_per_hour(&self) -> f64 {
        (self.meters_per_second / 1000.0) * 3600.0
    }

    /// Convert this speed to a floating point number of km/hour (kph)
    pub fn as_kilometres_per_hour(&self) -> f64 {
        self.as_kilometers_per_hour()
    }


    /// Convert this speed to a floating point number of miles/hour (mph)
    pub fn as_miles_per_hour(&self) -> f64 {
        (self.meters_per_second / 1609.0) * 3600.0
    }
}

impl Measurement for Speed {
    fn get_base_units(&self) -> f64 {
        self.meters_per_second
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_meters_per_second(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "m/s"
    }
}

implement_measurement! { Speed }
