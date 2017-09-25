//! Types and contants for handling masses.

use super::measurement::*;

// Constants, metric

/// Number of ng in a kg
pub const KILOGRAM_NANOGRAM_FACTOR: f64 = 1e12;
/// Number of µg in a kg
pub const KILOGRAM_MICROGRAM_FACTOR: f64 = 1e9;
/// Number of mg in a kg
pub const KILOGRAM_MILLIGRAM_FACTOR: f64 = 1e6;
/// Number of g in a kg
pub const KILOGRAM_GRAM_FACTOR: f64 = 1e3;
/// Number of Tonnes in a kg
pub const KILOGRAM_TONNE_FACTOR: f64 = 1e-3;
/// Number of carats in a kg
pub const KILOGRAM_CARAT_FACTOR: f64 = 5000.0;

// Constants, imperial

/// Number of Grains in a kg
pub const KILOGRAM_GRAINS_FACTOR: f64 = KILOGRAM_MILLIGRAM_FACTOR / 64.79891;
/// Number of Pennyweights in a kg
pub const KILOGRAM_PENNYWEIGHTS_FACTOR: f64 = KILOGRAM_GRAINS_FACTOR / 24.0;
/// Number of Avoirdupois Ounces in a kg
pub const KILOGRAM_OUNCES_FACTOR: f64 = KILOGRAM_POUNDS_FACTOR * 16.0;
/// Number of Troy Ounces in a kg
pub const KILOGRAM_TROY_OUNCES_FACTOR: f64 = KILOGRAM_GRAM_FACTOR / 31.1034768;
/// Number of Avoirdupois Pounds in a kg
pub const KILOGRAM_POUNDS_FACTOR: f64 = 1.0 / 0.45359237;
/// Number of Troy Pounds in a kg
pub const KILOGRAM_TROY_POUNDS_FACTOR: f64 = KILOGRAM_TROY_OUNCES_FACTOR / 12.0;
/// Number of Avoirdupois Stone in a kg
pub const KILOGRAM_STONES_FACTOR: f64 = KILOGRAM_POUNDS_FACTOR / 14.0;
/// Number of Short (US) Tons in a kg
pub const KILOGRAM_SHORT_TONS_FACTOR: f64 = KILOGRAM_POUNDS_FACTOR / 2000.0;
/// Number of Long (international) Tons in a kg
pub const KILOGRAM_LONG_TONS_FACTOR: f64 = KILOGRAM_POUNDS_FACTOR / 2240.0;

/// The Mass struct can be used to deal with mass in a common way. Metric,
/// avoirdupois imperial and troy imperial units are supported.
///
/// #Example
///
/// ```
/// use measurements::Mass;
///
/// let metric_ton = Mass::from_metric_tons(1.0);
/// let united_states_tons = metric_ton.as_short_tons();
/// let united_states_pounds = metric_ton.as_pounds();
/// println!(
///     "One metric ton is {} U.S. tons - that's {} pounds!",
///     united_states_tons, united_states_pounds);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Mass {
    kilograms: f64,
}

impl Mass {
    /// Create a Mass from a floating point value in kilograms
    pub fn from_kilograms(kilograms: f64) -> Self {
        Mass { kilograms: kilograms }
    }

    /// Create a Mass from a floating point value in micrograms
    pub fn from_micrograms(micrograms: f64) -> Self {
        Self::from_kilograms(micrograms / KILOGRAM_MICROGRAM_FACTOR)
    }

    /// Create a Mass from a floating point value in milligrams
    pub fn from_milligrams(milligrams: f64) -> Self {
        Self::from_kilograms(milligrams / KILOGRAM_MILLIGRAM_FACTOR)
    }

    /// Create a Mass from a floating point value in carats
    pub fn from_carats(carats: f64) -> Self {
        Self::from_kilograms(carats / KILOGRAM_CARAT_FACTOR)
    }

    /// Create a Mass from a floating point value in grams
    pub fn from_grams(grams: f64) -> Self {
        Self::from_kilograms(grams / KILOGRAM_GRAM_FACTOR)
    }

    /// Create a Mass from a floating point value in metric tonnes
    pub fn from_metric_tons(metric_tons: f64) -> Self {
        Self::from_kilograms(metric_tons / KILOGRAM_TONNE_FACTOR)
    }

    /// Create a Mass from a floating point value in metric tonnes
    pub fn from_tonnes(metric_tons: f64) -> Self {
        Self::from_kilograms(metric_tons / KILOGRAM_TONNE_FACTOR)
    }

    /// Create a Mass from a floating point value in grains
    pub fn from_grains(grains: f64) -> Self {
        Self::from_kilograms(grains / KILOGRAM_GRAINS_FACTOR)
    }

    /// Create a Mass from a floating point value in pennyweights
    pub fn from_pennyweights(pennyweights: f64) -> Self {
        Self::from_kilograms(pennyweights / KILOGRAM_PENNYWEIGHTS_FACTOR)
    }

    /// Create a Mass from a floating point value in ounces
    pub fn from_ounces(ounces: f64) -> Self {
        Self::from_kilograms(ounces / KILOGRAM_OUNCES_FACTOR)
    }

    /// Create a Mass from a floating point value in troy_ounces
    pub fn from_troy_ounces(troy_ounces: f64) -> Self {
        Self::from_kilograms(troy_ounces / KILOGRAM_TROY_OUNCES_FACTOR)
    }

    /// Create a Mass from a floating point value in Pounds (lbs)
    pub fn from_pounds(pounds: f64) -> Self {
        Self::from_kilograms(pounds / KILOGRAM_POUNDS_FACTOR)
    }

    /// Create a Mass from a floating point value in Troy Pounds
    pub fn from_troy_pounds(troy_pounds: f64) -> Self {
        Self::from_kilograms(troy_pounds / KILOGRAM_TROY_POUNDS_FACTOR)
    }

    /// Create a Mass from a floating point value in Stone (st.)
    pub fn from_stones(stones: f64) -> Self {
        Self::from_kilograms(stones / KILOGRAM_STONES_FACTOR)
    }

    /// Create a Mass from a floating point value in short (US) tons
    pub fn from_short_tons(short_tons: f64) -> Self {
        Self::from_kilograms(short_tons / KILOGRAM_SHORT_TONS_FACTOR)
    }

    /// Create a Mass from a floating point value in long (imperial) tons
    pub fn from_long_tons(long_tons: f64) -> Self {
        Self::from_kilograms(long_tons / KILOGRAM_LONG_TONS_FACTOR)
    }

    /// Convert this Mass to a floating point value in micrograms
    pub fn as_micrograms(&self) -> f64 {
        self.kilograms * KILOGRAM_MICROGRAM_FACTOR
    }

    /// Convert this Mass to a floating point value in milligrams
    pub fn as_milligrams(&self) -> f64 {
        self.kilograms * KILOGRAM_MILLIGRAM_FACTOR
    }

    /// Convert this Mass to a floating point value in carats
    pub fn as_carats(&self) -> f64 {
        self.kilograms * KILOGRAM_CARAT_FACTOR
    }

    /// Convert this Mass to a floating point value in grams
    pub fn as_grams(&self) -> f64 {
        self.kilograms * KILOGRAM_GRAM_FACTOR
    }

    /// Convert this Mass to a floating point value in kilograms (kg)
    pub fn as_kilograms(&self) -> f64 {
        self.kilograms
    }

    /// Convert this Mass to a floating point value in metric Tonnes
    pub fn as_metric_tons(&self) -> f64 {
        self.kilograms * KILOGRAM_TONNE_FACTOR
    }

    /// Convert this Mass to a floating point value in metric Tonnes
    pub fn as_tonnes(&self) -> f64 {
        self.kilograms * KILOGRAM_TONNE_FACTOR
    }

    /// Convert this Mass to a floating point value in Grains
    pub fn as_grains(&self) -> f64 {
        self.kilograms * KILOGRAM_GRAINS_FACTOR
    }

    /// Convert this Mass to a floating point value in Pennyweights
    pub fn as_pennyweights(&self) -> f64 {
        self.kilograms * KILOGRAM_PENNYWEIGHTS_FACTOR
    }

    /// Convert this Mass to a floating point value in Ounces (oz)
    pub fn as_ounces(&self) -> f64 {
        self.kilograms * KILOGRAM_OUNCES_FACTOR
    }

    /// Convert this Mass to a floating point value in Pounds (lbs)
    pub fn as_pounds(&self) -> f64 {
        self.kilograms * KILOGRAM_POUNDS_FACTOR
    }

    /// Convert this Mass to a floating point value in Troy Ounces
    pub fn as_troy_ounces(&self) -> f64 {
        self.kilograms * KILOGRAM_TROY_OUNCES_FACTOR
    }

    /// Convert this Mass to a floating point value in Troy Pounds
    pub fn as_troy_pounds(&self) -> f64 {
        self.kilograms * KILOGRAM_TROY_POUNDS_FACTOR
    }

    /// Convert this Mass to a floating point value in Stone (st.)
    pub fn as_stones(&self) -> f64 {
        self.kilograms * KILOGRAM_STONES_FACTOR
    }

    /// Convert this Mass to a floating point value in short (US) Tons
    pub fn as_short_tons(&self) -> f64 {
        self.kilograms * KILOGRAM_SHORT_TONS_FACTOR
    }

    /// Convert this Mass to a floating point value in long (international) Tons
    pub fn as_long_tons(&self) -> f64 {
        self.kilograms * KILOGRAM_LONG_TONS_FACTOR
    }
}

impl Measurement for Mass {
    fn get_base_units(&self) -> f64 {
        self.kilograms
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_kilograms(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "kg"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("ng", 1e-12),
            ("\u{00B5}g", 1e-9),
            ("mg", 1e-6),
            ("g", 1e-3),
            ("kg", 1e0),
            ("tonnes", 1e3),
            ("thousand tonnes", 1e6),
            ("million tonnes", 1e9),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Mass }
