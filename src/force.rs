use super::measurement::*;
use super::{Acceleration, Area, Mass, Pressure};

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


const POUNDS_PER_NEWTON: f64 = 0.224809;
const POUNDALS_PER_NEWTON: f64 = 7.2330;
const KILOPONDS_PER_NEWTON: f64 = 0.10197;
const DYNES_PER_NEWTON: f64 = 1e5;

impl Force {
    // Inputs, metric
    pub fn from_newtons(newtons: f64) -> Self {
        Force { newtons: newtons }
    }

    pub fn from_micronewtons(micronewtons: f64) -> Self {
        Self::from_newtons(micronewtons / 1e6)
    }

    pub fn from_millinewtons(millinewtons: f64) -> Self {
        Self::from_newtons(millinewtons / 1e3)
    }

    pub fn from_pounds(pounds: f64) -> Self {
        Self::from_newtons(pounds / POUNDS_PER_NEWTON)
    }

    pub fn from_poundals(poundals: f64) -> Self {
        Self::from_newtons(poundals / POUNDALS_PER_NEWTON)
    }

    pub fn from_kiloponds(kiloponds: f64) -> Self {
        Self::from_newtons(kiloponds / KILOPONDS_PER_NEWTON)
    }

    pub fn from_dynes(dynes: f64) -> Self {
        Self::from_newtons(dynes / DYNES_PER_NEWTON)
    }


    // Outputs, metric
    pub fn as_micronewtons(&self) -> f64 {
        self.newtons * 1e6
    }

    pub fn as_millinewtons(&self) -> f64 {
        self.newtons * 1e3
    }

    pub fn as_newtons(&self) -> f64 {
        self.newtons
    }

    pub fn as_pounds(&self) -> f64 {
        self.newtons * POUNDS_PER_NEWTON
    }

    pub fn as_poundals(&self) -> f64 {
        self.newtons * POUNDALS_PER_NEWTON
    }

    pub fn as_kiloponds(&self) -> f64 {
        self.newtons * KILOPONDS_PER_NEWTON
    }

    pub fn as_dynes(&self) -> f64 {
        self.newtons * DYNES_PER_NEWTON
    }
}

/// Force / Mass = Acceleration
impl ::std::ops::Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, rhs: Mass) -> Acceleration {
        Acceleration::from_meters_per_second_per_second(self.as_newtons() / rhs.as_kilograms())
    }
}

/// Force / Acceleration = Mass
impl ::std::ops::Div<Acceleration> for Force {
    type Output = Mass;

    fn div(self, rhs: Acceleration) -> Mass {
        Mass::from_kilograms(self.as_newtons() / rhs.as_meters_per_second_per_second())
    }
}

/// Force / Area = Pressure
impl ::std::ops::Div<Area> for Force {
    type Output = Pressure;

    fn div(self, rhs: Area) -> Pressure {
        Pressure::from_pascals(self.as_newtons() / rhs.as_square_metres())
    }
}

/// Force / Pressure = Area
impl ::std::ops::Div<Pressure> for Force {
    type Output = Area;

    fn div(self, rhs: Pressure) -> Area {
        Area::from_square_meters(self.as_newtons() / rhs.as_pascals())
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
