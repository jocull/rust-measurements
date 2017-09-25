use super::measurement::*;
use super::*;
use std::time::Duration;

/// The `Energy` struct can be used to deal with energies in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::Energy;
///
/// let energy = Energy::from_kcalories(2500.0);
/// println!("Some say a health adult male should consume {} per day", energy);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Energy {
    joules: f64,
}

impl Energy {
    pub fn from_joules(joules: f64) -> Energy {
        Energy { joules: joules }
    }

    pub fn from_kcalories(kcalories: f64) -> Energy {
        Self::from_joules(kcalories * 4186.8)
    }

    pub fn from_btu(btu: f64) -> Energy {
        Self::from_joules(btu * 1055.056)
    }

    pub fn from_e_v(e_v: f64) -> Energy {
        Self::from_joules(e_v / 6.241509479607718e+18)
    }

    pub fn from_watt_hours(wh: f64) -> Energy {
        Self::from_joules(wh * 3600.0)
    }

    pub fn from_kilowatt_hours(kwh: f64) -> Energy {
        Self::from_joules(kwh * 3600.0 * 1000.0)
    }

    pub fn as_joules(&self) -> f64 {
        self.joules
    }

    pub fn as_kcalories(&self) -> f64 {
        self.joules / 4186.8
    }

    pub fn as_btu(&self) -> f64 {
        self.joules / 1055.056
    }

    pub fn as_e_v(&self) -> f64 {
        self.joules * 6.241509479607718e+18
    }

    pub fn as_watt_hours(&self) -> f64 {
        self.joules / 3600.0
    }

    pub fn as_kilowatt_hours(&self) -> f64 {
        self.joules / (3600.0 * 1000.0)
    }
}

/// Energy / Time = Power
impl ::std::ops::Div<Duration> for Energy {
    type Output = Power;

    fn div(self, rhs: Duration) -> Power {
        Power::from_watts(self.as_joules() / duration_as_f64(rhs))
    }
}

/// Energy / Power = Time
impl ::std::ops::Div<Power> for Energy {
    type Output = Duration;

    fn div(self, rhs: Power) -> Duration {
        let seconds = self.as_joules() / rhs.as_watts();
        let nanosecs = (seconds * 1e9) % 1e9;
        Duration::new(seconds as u64, nanosecs as u32)
    }
}

/// Energy / Length = Force
impl ::std::ops::Div<Length> for Energy {
    type Output = Force;

    fn div(self, rhs: Length) -> Force {
        Force::from_newtons(self.as_joules() / rhs.as_meters())
    }
}

/// Energy / Force = Length
impl ::std::ops::Div<Force> for Energy {
    type Output = Length;

    fn div(self, rhs: Force) -> Length {
        Length::from_meters(self.as_joules() / rhs.as_newtons())
    }
}

impl Measurement for Energy {
    fn get_base_units(&self) -> f64 {
        self.joules
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_joules(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "J"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to Largest
        let list = [
            ("fJ", 1e-15),
            ("pJ", 1e-12),
            ("nJ", 1e-9),
            ("\u{00B5}J", 1e-6),
            ("mJ", 1e-3),
            ("J", 1e0),
            ("kJ", 1e3),
            ("MJ", 1e6),
            ("GJ", 1e9),
            ("TJ", 1e12),
            ("PJ", 1e15),
            ("EJ", 1e18),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Energy }
