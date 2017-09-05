use super::measurement::*;
use super::Length;
use super::Acceleration;
use std::time::Duration;

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
    pub fn from_meters_per_second(meters_per_second: f64) -> Speed {
        Speed { meters_per_second: meters_per_second }
    }

    pub fn from_metres_per_second(metres_per_second: f64) -> Speed {
        Speed::from_meters_per_second(metres_per_second)
    }

    pub fn from_kilometers_per_hour(kilometers_per_hour: f64) -> Speed {
        Speed::from_meters_per_second((kilometers_per_hour * 1000.0) / 3600.0)
    }

    pub fn from_kilometres_per_hour(kilometres_per_hour: f64) -> Speed {
        Speed::from_kilometers_per_hour(kilometres_per_hour)
    }

    pub fn from_miles_per_hour(miles_per_hour: f64) -> Speed {
        Speed::from_meters_per_second((miles_per_hour * 1609.0) / 3600.0)
    }

    pub fn as_meters_per_second(&self) -> f64 {
        self.meters_per_second
    }

    pub fn as_metres_per_second(&self) -> f64 {
        self.as_meters_per_second()
    }

    pub fn as_kilometers_per_hour(&self) -> f64 {
        (self.meters_per_second / 1000.0) * 3600.0
    }

    pub fn as_kilometres_per_hour(&self) -> f64 {
        self.as_kilometers_per_hour()
    }


    pub fn as_miles_per_hour(&self) -> f64 {
        (self.meters_per_second / 1609.0) * 3600.0
    }
}

/// Speed / Acceleration = Time
impl ::std::ops::Div<Acceleration> for Speed {
    type Output = Duration;

    fn div(self, rhs: Acceleration) -> Duration {
        let seconds =  self.as_meters_per_second() / rhs.as_meters_per_second_per_second();
        let nanosecs = (seconds * 1e9) % 1e9;
        Duration::new(seconds as u64, nanosecs as u32)
    }
}

/// Speed / Time = Acceleration
impl ::std::ops::Div<Duration> for Speed {
    type Output = Acceleration;

    fn div(self, rhs: Duration) -> Acceleration {
        // It would be useful if Duration had a method that did this...
        let seconds: f64 = rhs.as_secs() as f64 + ((rhs.subsec_nanos() as f64) * 1e-9);
        Acceleration::from_meters_per_second_per_second(self.as_meters_per_second() / seconds)
    }
}

/// Speed * Time = Length
impl ::std::ops::Mul<Duration> for Speed {
    type Output = Length;

    fn mul(self, rhs: Duration) -> Length {
        // It would be useful if Duration had a method that did this...
        let seconds: f64 = rhs.as_secs() as f64 + ((rhs.subsec_nanos() as f64) * 1e-9);
        Length::from_meters(self.as_meters_per_second() * seconds)
    }
}

/// Time * Speed = Length
impl ::std::ops::Mul<Speed> for Duration {
    type Output = Length;

    fn mul(self, rhs: Speed) -> Length {
        rhs * self
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
