use super::measurement::*;
use super::*;
use std::time::Duration;

/// The `Power` struct can be used to deal with energies in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::Power;
///
/// let power = Power::from_horsepower(100.0);
/// let k_w = power.as_kilowatts();
/// println!("A 100.0 hp car produces {} kW", k_w);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Power {
    watts: f64,
}

impl Power {
    pub fn from_watts(watts: f64) -> Power {
        Power { watts: watts }
    }

    pub fn from_horsepower(horsepower: f64) -> Power {
        Self::from_watts(horsepower * 745.6998715822702)
    }

    pub fn from_btu_per_minute(btu_per_minute: f64) -> Power {
        Self::from_watts(btu_per_minute * 17.58426666666667)
    }

    pub fn from_kilowatts(kw: f64) -> Power {
        Self::from_watts(kw * 1000.0)
    }

    pub fn as_watts(&self) -> f64 {
        self.watts
    }

    pub fn as_horsepower(&self) -> f64 {
        self.watts / 745.6998715822702
    }

    pub fn as_btu_per_minute(&self) -> f64 {
        self.watts / 17.58426666666667
    }

    pub fn as_kilowatts(&self) -> f64 {
        self.watts / 1000.0
    }
}

/// Power * Time = Energy
impl ::std::ops::Mul<Duration> for Power {
    type Output = Energy;

    fn mul(self, rhs: Duration) -> Energy {
        Energy::from_joules(self.as_watts() * duration_as_f64(rhs))
    }
}

/// Time * Power = Energy
impl ::std::ops::Mul<Power> for Duration {
    type Output = Energy;

    fn mul(self, rhs: Power) -> Energy {
        rhs * self
    }
}

impl Measurement for Power {
    fn get_base_units(&self) -> f64 {
        self.watts
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_watts(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "W"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to Largest
        let list = [
            ("fW", 1e-15),
            ("pW", 1e-12),
            ("nW", 1e-9),
            ("\u{00B5}W", 1e-6),
            ("mW", 1e-3),
            ("W", 1e0),
            ("kW", 1e3),
            ("MW", 1e6),
            ("GW", 1e9),
            ("TW", 1e12),
            ("PW", 1e15),
            ("EW", 1e18),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Power }
