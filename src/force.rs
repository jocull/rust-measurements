//! Types and contants for handling force.

use super::measurement::*;

/// Number of POUNDS in a Newton
pub const POUNDS_PER_NEWTON: f64 = 0.224809;
/// Number of POUNDALS in a Newton
pub const POUNDALS_PER_NEWTON: f64 = 7.2330;
/// Number of KILOPONDS in a Newton
pub const KILOPONDS_PER_NEWTON: f64 = 0.10197;
/// Number of DYNES in a Newton
pub const DYNES_PER_NEWTON: f64 = 1e5;

/// The `Force` struct can be used to deal with force in a common way.
///
/// #Example
///
/// ```
/// use measurements::Force;
/// use measurements::Mass;
/// use measurements::Acceleration;
///
/// let metric_ton = Mass::from_metric_tons(1.0);
/// let gravity = Acceleration::from_meters_per_second_per_second(9.81);
/// let force = metric_ton * gravity; // F=ma
/// println!(
///     "One metric ton exerts a force of {} due to gravity",
///     force);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Force {
    newtons: f64,
}

impl Force {
    /// Create a Force from a floating point value in Newtons
    pub fn from_newtons(newtons: f64) -> Self {
        Force { newtons: newtons }
    }

    /// Create a Force from a floating point value in Micronewtons
    pub fn from_micronewtons(micronewtons: f64) -> Self {
        Self::from_newtons(micronewtons / 1e6)
    }

    /// Create a Force from a floating point value in Millinewtons
    pub fn from_millinewtons(millinewtons: f64) -> Self {
        Self::from_newtons(millinewtons / 1e3)
    }

    /// Create a Force from a floating point value in pounds
    pub fn from_pounds(pounds: f64) -> Self {
        Self::from_newtons(pounds / POUNDS_PER_NEWTON)
    }

    /// Create a Force from a floating point value in poundals
    pub fn from_poundals(poundals: f64) -> Self {
        Self::from_newtons(poundals / POUNDALS_PER_NEWTON)
    }

    /// Create a Force from a floating point value in kiloponds
    pub fn from_kiloponds(kiloponds: f64) -> Self {
        Self::from_newtons(kiloponds / KILOPONDS_PER_NEWTON)
    }

    /// Create a Force from a floating point value in Dynes
    pub fn from_dynes(dynes: f64) -> Self {
        Self::from_newtons(dynes / DYNES_PER_NEWTON)
    }


    /// Convert this Force into a floating point value in Micronewtons
    pub fn as_micronewtons(&self) -> f64 {
        self.newtons * 1e6
    }

    /// Convert this Force into a floating point value in Milliewtons
    pub fn as_millinewtons(&self) -> f64 {
        self.newtons * 1e3
    }

    /// Convert this Force into a floating point value in Newtons
    pub fn as_newtons(&self) -> f64 {
        self.newtons
    }

    /// Convert this Force into a floating point value in pound-force (lb.f)
    pub fn as_pounds(&self) -> f64 {
        self.newtons * POUNDS_PER_NEWTON
    }

    /// Convert this Force into a floating point value in poundals
    pub fn as_poundals(&self) -> f64 {
        self.newtons * POUNDALS_PER_NEWTON
    }

    /// Convert this Force into a floating point value in kiloponds
    pub fn as_kiloponds(&self) -> f64 {
        self.newtons * KILOPONDS_PER_NEWTON
    }

    /// Convert this Force into a floating point value in dynes
    pub fn as_dynes(&self) -> f64 {
        self.newtons * DYNES_PER_NEWTON
    }
}

impl Measurement for Force {
    fn get_base_units(&self) -> f64 {
        self.newtons
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_newtons(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "N"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("nN", 1e-9),
            ("\u{00B5}N", 1e-6),
            ("mN", 1e-3),
            ("N", 1e0),
            ("kN", 1e3),
            ("MN", 1e6),
            ("GN", 1e9),
            ("TN", 1e12),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Force }
