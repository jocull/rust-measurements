use super::measurement::*;
use super::*;

/// The `Pressure` struct can be used to deal with presssures in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::Pressure;
///
/// let earth = Pressure::from_atmospheres(1.0);
/// let mbar = earth.as_millibars();
/// println!("Atmospheric pressure is {} mbar.", mbar);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Pressure {
    pascals: f64,
}

impl Pressure {
    pub fn from_pascals(pascals: f64) -> Pressure {
        Pressure { pascals: pascals }
    }

    pub fn from_hectopascals(hectopascals: f64) -> Pressure {
        Self::from_pascals(hectopascals * 100.0)
    }

    pub fn from_millibars(millibars: f64) -> Pressure {
        Self::from_pascals(millibars * 100.0)
    }

    pub fn from_kilopascals(kilopascals: f64) -> Pressure {
        Self::from_pascals(kilopascals * 1000.0)
    }

    pub fn from_psi(psi: f64) -> Pressure {
        Self::from_pascals(psi * 6894.76)
    }

    pub fn from_bars(bars: f64) -> Pressure {
        Self::from_pascals(bars * 100_000.0)
    }

    pub fn from_atmospheres(atmospheres: f64) -> Pressure {
        Self::from_pascals(atmospheres * 101_325.0)
    }

    pub fn as_pascals(&self) -> f64 {
        self.pascals
    }

    pub fn as_hectopascals(&self) -> f64 {
        self.pascals / 100.0
    }

    pub fn as_millibars(&self) -> f64 {
        self.pascals / 100.0
    }

    pub fn as_kilopascals(&self) -> f64 {
        self.pascals / 1000.0
    }

    pub fn as_psi(&self) -> f64 {
        self.pascals / 6894.76
    }

    pub fn as_bars(&self) -> f64 {
        self.pascals / 100_000.0
    }

    pub fn as_atmospheres(&self) -> f64 {
        self.pascals / 101_325.0
    }
}

/// Pressure * Area = Force
impl ::std::ops::Mul<Area> for Pressure {
    type Output = Force;

    fn mul(self, rhs: Area) -> Force {
        Force::from_newtons(rhs.as_square_meters() * self.as_pascals())
    }
}

impl Measurement for Pressure {
    fn get_base_units(&self) -> f64 {
        self.pascals
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_pascals(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "Pa"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        let list = [
            ("mPa", 1e-3),
            ("Pa", 1e0),
            ("hPa", 1e2),
            ("kPa", 1e3),
            ("MPa", 1e6),
            ("GPa", 1e9),
            ("TPa", 1e12),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Pressure }
