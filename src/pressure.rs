use super::measurement::*;

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
    millibars: f64,
}

impl Pressure {
    pub fn from_hectopascals(hectopascals: f64) -> Pressure {
        Self::from_millibars(hectopascals)
    }

    pub fn from_pascals(pascals: f64) -> Pressure {
        Self::from_millibars(pascals / 100.0)
    }

    pub fn from_kilopascals(kilopascals: f64) -> Pressure {
        Self::from_millibars(kilopascals * 10.0)
    }

    pub fn from_millibars(millibars: f64) -> Pressure {
        Pressure { millibars: millibars }
    }

    pub fn from_bars(bars: f64) -> Pressure {
        Self::from_millibars(bars * 1000.0)
    }

    pub fn from_psi(psi: f64) -> Pressure {
        Self::from_millibars(psi / 0.0145038)
    }

    pub fn from_atmospheres(atmospheres: f64) -> Pressure {
        Self::from_millibars(atmospheres * 1013.25)
    }

    pub fn as_hectopascals(&self) -> f64 {
        self.millibars
    }

    pub fn as_pascals(&self) -> f64 {
        self.millibars * 100.0
    }

    pub fn as_kilopascals(&self) -> f64 {
        self.millibars / 10.0
    }

    pub fn as_millibars(&self) -> f64 {
        self.millibars
    }

    pub fn as_bars(&self) -> f64 {
        self.millibars / 1000.0
    }

    pub fn as_psi(&self) -> f64 {
        self.millibars * 0.0145038
    }

    pub fn as_atmospheres(&self) -> f64 {
        self.millibars / 1013.25
    }
}

impl Measurement for Pressure {
    fn get_base_units(&self) -> f64 {
        self.millibars
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_millibars(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "mbar"
    }
}

implement_measurement! { Pressure }

