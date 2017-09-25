//! Types and contants for handling temperature.

use super::measurement::*;

/// The `Temperature` struct can be used to deal with absolute temperatures in
/// a common way.
///
/// # Example
///
/// ```
/// use measurements::Temperature;
///
/// let boiling_water = Temperature::from_celsius(100.0);
/// let fahrenheit = boiling_water.as_fahrenheit();
/// println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Temperature {
    degrees_kelvin: f64,
}

/// The `TemperatureDelta` struct can be used to deal with differences between
/// temperatures in a common way.
///
/// # Example
///
/// ```
/// use measurements::{Temperature, TemperatureDelta};
///
/// let boiling_water = Temperature::from_celsius(100.0);
/// let frozen_water = Temperature::from_celsius(0.0);
/// let difference = boiling_water - frozen_water;
/// println!("Boiling water is {} above freezing.", difference);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct TemperatureDelta {
    kelvin_degrees: f64,
}

impl TemperatureDelta {
    /// Create a new TemperatureDelta from a floating point value in Kelvin
    pub fn from_kelvin(kelvin_degrees: f64) -> Self {
        TemperatureDelta { kelvin_degrees: kelvin_degrees }
    }

    /// Create a new TemperatureDelta from a floating point value in Celsius
    pub fn from_celsius(celsius_degrees: f64) -> Self {
        TemperatureDelta::from_kelvin(celsius_degrees)
    }

    /// Create a new TemperatureDelta from a floating point value in Fahrenheit
    pub fn from_fahrenheit(farenheit_degrees: f64) -> Self {
        TemperatureDelta { kelvin_degrees: farenheit_degrees / 1.8 }
    }

    /// Create a new TemperatureDelta from a floating point value in Rankine
    pub fn from_rankine(rankine_degrees: f64) -> Self {
        TemperatureDelta { kelvin_degrees: rankine_degrees / 1.8 }
    }

    /// Convert this TemperatureDelta to a floating point value in Kelvin
    pub fn as_kelvin(&self) -> f64 {
        self.kelvin_degrees
    }

    /// Convert this TemperatureDelta to a floating point value in Celsius
    pub fn as_celsius(&self) -> f64 {
        self.kelvin_degrees
    }

    /// Convert this TemperatureDelta to a floating point value in Fahrenheit
    pub fn as_fahrenheit(&self) -> f64 {
        self.kelvin_degrees * 1.8
    }

    /// Convert this TemperatureDelta to a floating point value in Rankine
    pub fn as_rankine(&self) -> f64 {
        self.kelvin_degrees * 1.8
    }
}

impl Temperature {
    /// Create a new Temperature from a floating point value in Kelvin
    pub fn from_kelvin(degrees_kelvin: f64) -> Self {
        Temperature { degrees_kelvin: degrees_kelvin }
    }

    /// Create a new Temperature from a floating point value in Celsius
    pub fn from_celsius(degrees_celsius: f64) -> Self {
        Self::from_kelvin(degrees_celsius + 273.15)
    }

    /// Create a new Temperature from a floating point value in Fahrenheit
    pub fn from_fahrenheit(degrees_fahrenheit: f64) -> Self {
        Self::from_kelvin((degrees_fahrenheit - 32.0) / 1.8 + 273.15)
    }

    /// Create a new Temperature from a floating point value in Rankine
    pub fn from_rankine(degrees_rankine: f64) -> Self {
        Self::from_kelvin((degrees_rankine - 491.67) / 1.8 + 273.15)
    }

    /// Convert this absolute Temperature to a floating point value in Kelvin
    pub fn as_kelvin(&self) -> f64 {
        self.degrees_kelvin
    }

    /// Convert this absolute Temperature to a floating point value in Celsius
    pub fn as_celsius(&self) -> f64 {
        self.degrees_kelvin - 273.15
    }

    /// Convert this absolute Temperature to a floating point value in Fahrenheit
    pub fn as_fahrenheit(&self) -> f64 {
        (self.degrees_kelvin - 273.15) * 1.8 + 32.0
    }

    /// Convert this absolute Temperature to a floating point value in Rankine
    pub fn as_rankine(&self) -> f64 {
        (self.degrees_kelvin - 273.15) * 1.8 + 491.67
    }
}

impl Measurement for Temperature {
    fn get_base_units(&self) -> f64 {
        self.degrees_kelvin
    }

    fn from_base_units(degrees_kelvin: f64) -> Self {
        Self::from_kelvin(degrees_kelvin)
    }

    fn get_base_units_name(&self) -> &'static str {
        "K"
    }
}

impl Measurement for TemperatureDelta {
    fn get_base_units(&self) -> f64 {
        self.kelvin_degrees
    }

    fn from_base_units(kelvin_degrees: f64) -> Self {
        Self::from_kelvin(kelvin_degrees)
    }

    fn get_base_units_name(&self) -> &'static str {
        "K"
    }
}

impl ::std::ops::Add<TemperatureDelta> for Temperature {
    type Output = Temperature;

    fn add(self, other: TemperatureDelta) -> Temperature {
        Temperature::from_kelvin(self.degrees_kelvin + other.kelvin_degrees)
    }
}

impl ::std::ops::Add<Temperature> for TemperatureDelta {
    type Output = Temperature;

    fn add(self, other: Temperature) -> Temperature {
        other + self
    }
}

impl ::std::ops::Sub<TemperatureDelta> for Temperature {
    type Output = Temperature;

    fn sub(self, other: TemperatureDelta) -> Temperature {
        Temperature::from_kelvin(self.degrees_kelvin - other.kelvin_degrees)
    }
}

impl ::std::ops::Sub<Temperature> for Temperature {
    type Output = TemperatureDelta;

    fn sub(self, other: Temperature) -> TemperatureDelta {
        TemperatureDelta::from_kelvin(self.degrees_kelvin - other.degrees_kelvin)
    }
}

impl ::std::cmp::Eq for Temperature {}
impl ::std::cmp::PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        self.get_base_units() == other.get_base_units()
    }
}

impl ::std::cmp::PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.get_base_units().partial_cmp(&other.get_base_units())
    }
}

implement_display!(Temperature);
implement_measurement!(TemperatureDelta);
