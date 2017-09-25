//! Types and contants for handling power.

use super::measurement::*;

/// Number of horsepower in a watt
pub const WATT_HORSEPOWER_FACTOR: f64 = 1.0 / 745.6998715822702;
/// Number of BTU/min in a watt
pub const WATT_BTU_MIN_FACTOR: f64 = 1.0 / 17.58426666666667;
/// Number of kW in a W
pub const WATT_KILOWATT_FACTOR: f64 = 1e-3;
/// Number of pferdstarken (PS) in a W
pub const WATT_PS_FACTOR: f64 = 1.0 / 735.499;

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
    /// Create a new Power from a floating point value in Watts
    pub fn from_watts(watts: f64) -> Power {
        Power { watts: watts }
    }

    /// Create a new Power from a floating point value in horsepower (hp)
    pub fn from_horsepower(horsepower: f64) -> Power {
        Self::from_watts(horsepower / WATT_HORSEPOWER_FACTOR)
    }

    /// Create a new Power from a floating point value in metric horsepower (PS)
    pub fn from_ps(ps: f64) -> Power {
        Self::from_watts(ps / WATT_PS_FACTOR)
    }

    /// Create a new Power from a floating point value in metric horsepower (PS)
    pub fn from_metric_horsepower(metric_horsepower: f64) -> Power {
        Self::from_watts(metric_horsepower / WATT_PS_FACTOR)
    }

    /// Create a new Power from a floating point value in BTU/mjn
    pub fn from_btu_per_minute(btu_per_minute: f64) -> Power {
        Self::from_watts(btu_per_minute / WATT_BTU_MIN_FACTOR)
    }

    /// Create a new Power from a floating point value in Kilowatts (kW)
    pub fn from_kilowatts(kw: f64) -> Power {
        Self::from_watts(kw / WATT_KILOWATT_FACTOR)
    }

    /// Convert this Power into a floating point value in Watts
    pub fn as_watts(&self) -> f64 {
        self.watts
    }

    /// Convert this Power into a floating point value in horsepower (hp)
    pub fn as_horsepower(&self) -> f64 {
        self.watts * WATT_HORSEPOWER_FACTOR
    }

    /// Convert this Power into a floating point value in metric horsepower (PS)
    pub fn as_ps(&self) -> f64 {
        self.watts * WATT_PS_FACTOR
    }

    /// Convert this Power into a floating point value in metric horsepower (PS)
    pub fn as_metric_horsepower(&self) -> f64 {
        self.watts * WATT_PS_FACTOR
    }

    /// Convert this Power into a floating point value in BTU/min
    pub fn as_btu_per_minute(&self) -> f64 {
        self.watts * WATT_BTU_MIN_FACTOR
    }

    /// Convert this Power into a floating point value in kilowatts (kW)
    pub fn as_kilowatts(&self) -> f64 {
        self.watts * WATT_KILOWATT_FACTOR
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
