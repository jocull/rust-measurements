//! Types and contants for handling volumes (that is, three-dimensional space, not loudness).

use super::measurement::*;

/// The `Volume` struct can be used to deal with volumes in a common way.
///
/// #Example
///
/// ```
/// use measurements::Volume;
///
/// let gallon = Volume::from_gallons(1.0);
/// let pint = Volume::from_pints(1.0);
/// let beers = gallon / pint;
/// println!("A gallon of beer will pour {} pints!", beers);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Volume {
    liters: f64,
}

/// Number of Milliliters in a litre
pub const LITER_MILLILITERS_FACTOR: f64 = 1000.0;
/// Number of Cubic Centimeters in a litre
pub const LITER_CUBIC_CENTIMETER_FACTOR: f64 = 1000.0;
/// Number of Cubic Meters in a litre
pub const LITER_CUBIC_METER_FACTOR: f64 = 1.0 / 1000.0;
/// Number of Drops in a litre
pub const LITER_DROP_FACTOR: f64 = 15419.6298055;
/// Number of (US) Fluid Drams in a litre
pub const LITER_DRAM_FACTOR: f64 = 270.510351863;
/// Number of Teaspoons in a litre
pub const LITER_TEASPOONS_FACTOR: f64 = 202.8841362;
/// Number of Tablespoons in a litre
pub const LITER_TABLESPOONS_FACTOR: f64 = 67.6280454;
/// Number of Cubic Inches in a litre
pub const LITER_CUBIC_INCHES_FACTOR: f64 = 61.0237440947;
/// Number of Fluid Ounces (UK) in a litre
pub const LITER_FLUID_OUNCES_UK_FACTOR: f64 = 35.19506424;
/// Number of Fluid Ounces (US) in a litre
pub const LITER_FLUID_OUNCES_FACTOR: f64 = 33.8140227;
/// Number of Cups in a litre
pub const LITER_CUP_FACTOR: f64 = 4.226752838;
/// Number of Pints in a litre
pub const LITER_PINTS_FACTOR: f64 = 2.11337641887;
/// Number of Pints (UK) in a litre
pub const LITER_PINTS_UK_FACTOR: f64 = 1.75975398639;
/// Number of Quarts in a litre
pub const LITER_QUARTS_FACTOR: f64 = 1.05668820943;
/// Number of Gallons in a litre
pub const LITER_GALLONS_FACTOR: f64 = 0.264172052358;
/// Number of Gallons (UK) in a litre
pub const LITER_GALLONS_UK_FACTOR: f64 = 0.219969248299;
/// Number of Cubic Feet in a litre
pub const LITER_CUBIC_FEET_FACTOR: f64 = 0.0353146667215;
/// Number of Cubic Yards in a litre
pub const LITER_CUBIC_YARD_FACTOR: f64 = 0.0013079506193;

impl Volume {
    /// Create a new Volume from a floating point value in Liters (l)
    pub fn from_liters(liters: f64) -> Self {
        Volume { liters: liters }
    }

    /// Create a new Volume from a floating point value in Litres (l)
    pub fn from_litres(liters: f64) -> Self {
        Self::from_liters(liters)
    }

    /// Create a new Volume from a floating point value in Cubic Centimeters (cc or cm³)
    pub fn from_cubic_centimeters(cubic_centimeters: f64) -> Self {
        Self::from_liters(cubic_centimeters / LITER_CUBIC_CENTIMETER_FACTOR)
    }

    /// Create a new Volume from a floating point value in Cubic Centimetres (cc or cm³)
    pub fn from_cubic_centimetres(cubic_centimeters: f64) -> Self {
        Self::from_cubic_centimeters(cubic_centimeters)
    }

    /// Create a new Volume from a floating point value in Milliliters (ml)
    pub fn from_milliliters(milliliters: f64) -> Self {
        Self::from_liters(milliliters / LITER_MILLILITERS_FACTOR)
    }

    /// Create a new Volume from a floating point value in Millilitres (ml)
    pub fn from_millilitres(milliliters: f64) -> Self {
        Self::from_milliliters(milliliters)
    }

    /// Create a new Volume from a floating point value in Cubic Meters (m³)
    pub fn from_cubic_meters(cubic_meters: f64) -> Self {
        Self::from_liters(cubic_meters / LITER_CUBIC_METER_FACTOR)
    }

    /// Create a new Volume from a floating point value in Cubic Metres (m³)
    pub fn from_cubic_metres(cubic_meters: f64) -> Self {
        Self::from_cubic_meters(cubic_meters)
    }

    /// Create a new Volume from a floating point value in Drops
    pub fn from_drops(drops: f64) -> Self {
        Self::from_liters(drops / LITER_DROP_FACTOR)
    }

    /// Create a new Volume from a floating point value in US Fluid Drams
    pub fn from_drams(drams: f64) -> Self {
        Self::from_liters(drams / LITER_DRAM_FACTOR)
    }

    /// Create a new Volume from a floating point value in Teaspoons (tsp)
    pub fn from_teaspoons(teaspoons: f64) -> Self {
        Self::from_liters(teaspoons / LITER_TEASPOONS_FACTOR)
    }

    /// Create a new Volume from a floating point value in Tablespoons (tbsp)
    pub fn from_tablespoons(tablespoons: f64) -> Self {
        Self::from_liters(tablespoons / LITER_TABLESPOONS_FACTOR)
    }

    /// Create a new Volume from a floating point value in UK Fluid Ounces (fl oz)
    pub fn from_fluid_ounces_uk(fluid_ounces_uk: f64) -> Self {
        Self::from_liters(fluid_ounces_uk / LITER_FLUID_OUNCES_UK_FACTOR)
    }

    /// Create a new Volume from a floating point value in US Fluid Ounces (fl oz)
    pub fn from_fluid_ounces(fluid_ounces: f64) -> Self {
        Self::from_liters(fluid_ounces / LITER_FLUID_OUNCES_FACTOR)
    }

    /// Create a new Volume from a floating point value in Cubic Inches (cu in or in³)
    pub fn from_cubic_inches(cubic_inches: f64) -> Self {
        Self::from_liters(cubic_inches / LITER_CUBIC_INCHES_FACTOR)
    }

    /// Create a new Volume from a floating point value in Cups
    pub fn from_cups(cups: f64) -> Self {
        Self::from_liters(cups / LITER_CUP_FACTOR)
    }

    /// Create a new Volume from a floating point value in US Pints
    pub fn from_pints(pints: f64) -> Self {
        Self::from_liters(pints / LITER_PINTS_FACTOR)
    }

    /// Create a new Volume from a floating point value in UK Pints
    pub fn from_pints_uk(pints_uk: f64) -> Self {
        Self::from_liters(pints_uk / LITER_PINTS_UK_FACTOR)
    }

    /// Create a new Volume from a floating point value in Quarts
    pub fn from_quarts(quarts: f64) -> Self {
        Self::from_liters(quarts / LITER_QUARTS_FACTOR)
    }

    /// Create a new Volume from a floating point value in US Gallons (gal US)
    pub fn from_gallons(gallons: f64) -> Self {
        Self::from_liters(gallons / LITER_GALLONS_FACTOR)
    }

    /// Create a new Volume from a floating point value in UK/Imperial Gallons (gal)
    pub fn from_gallons_uk(gallons_uk: f64) -> Self {
        Self::from_liters(gallons_uk / LITER_GALLONS_UK_FACTOR)
    }

    /// Create a new Volume from a floating point value in Cubic Feet (ft³)
    pub fn from_cubic_feet(cubic_feet: f64) -> Self {
        Self::from_liters(cubic_feet / LITER_CUBIC_FEET_FACTOR)
    }

    /// Create a new Volume from a floating point value in Cubic Yards (yd³)
    pub fn from_cubic_yards(cubic_yards: f64) -> Self {
        Self::from_liters(cubic_yards / LITER_CUBIC_YARD_FACTOR)
    }

    /// Convert Volume to a floating point value in Cubic Centimeters (cc or cm³)
    pub fn as_cubic_centimeters(&self) -> f64 {
        self.liters * LITER_CUBIC_CENTIMETER_FACTOR
    }

    /// Convert Volume to a floating point value in Cubic Centimetres (cc or cm³)
    pub fn as_cubic_centimetres(&self) -> f64 {
        self.as_cubic_centimeters()
    }

    /// Convert Volume to a floating point value in Milliliters (ml)
    pub fn as_milliliters(&self) -> f64 {
        self.liters * LITER_MILLILITERS_FACTOR
    }

    /// Convert Volume to a floating point value in Millilitres (ml)
    pub fn as_millilitres(&self) -> f64 {
        self.as_milliliters()
    }

    /// Convert Volume to a floating point value in Liters (l)
    pub fn as_liters(&self) -> f64 {
        self.liters
    }

    /// Convert Volume to a floating point value in Litres (l)
    pub fn as_litres(&self) -> f64 {
        self.as_liters()
    }

    /// Convert Volume to a floating point value in Cubic Meters (m³)
    pub fn as_cubic_meters(&self) -> f64 {
        self.liters * LITER_CUBIC_METER_FACTOR
    }

    /// Convert Volume to a floating point value in Cubic Metres (m³)
    pub fn as_cubic_metres(&self) -> f64 {
        self.as_cubic_meters()
    }

    /// Convert Volume to a floating point value in Drops
    pub fn as_drops(&self) -> f64 {
        self.liters * LITER_DROP_FACTOR
    }

    /// Convert Volume to a floating point value in US Fluid Drams
    pub fn as_drams(&self) -> f64 {
        self.liters * LITER_DRAM_FACTOR
    }

    /// Convert Volume to a floating point value in Teaspoons (tsp)
    pub fn as_teaspoons(&self) -> f64 {
        self.liters * LITER_TEASPOONS_FACTOR
    }

    /// Convert Volume to a floating point value in Tablespoons (tbsp)
    pub fn as_tablespoons(&self) -> f64 {
        self.liters * LITER_TABLESPOONS_FACTOR
    }

    /// Convert Volume to a floating point value in Cubic Inches (cu in or in³)
    pub fn as_cubic_inches(&self) -> f64 {
        self.liters * LITER_CUBIC_INCHES_FACTOR
    }

    /// Convert Volume to a floating point value in UK Fluid Ounces (fl oz)
    pub fn as_fluid_ounces_uk(&self) -> f64 {
        self.liters * LITER_FLUID_OUNCES_UK_FACTOR
    }

    /// Convert Volume to a floating point value in US Fluid Ounces (fl oz)
    pub fn as_fluid_ounces(&self) -> f64 {
        self.liters * LITER_FLUID_OUNCES_FACTOR
    }

    /// Convert Volume to a floating point value in Cups
    pub fn as_cups(&self) -> f64 {
        self.liters * LITER_CUP_FACTOR
    }

    /// Convert Volume to a floating point value in US Pints
    pub fn as_pints(&self) -> f64 {
        self.liters * LITER_PINTS_FACTOR
    }

    /// Convert Volume to a floating point value in UK Pints
    pub fn as_pints_uk(&self) -> f64 {
        self.liters * LITER_PINTS_UK_FACTOR
    }

    /// Convert Volume to a floating point value in Quarts
    pub fn as_quarts(&self) -> f64 {
        self.liters * LITER_QUARTS_FACTOR
    }

    /// Convert Volume to a floating point value in US Gallons (gal us)
    pub fn as_gallons(&self) -> f64 {
        self.liters * LITER_GALLONS_FACTOR
    }

    /// Convert Volume to a floating point value in UK/Imperial Gallons (gal)
    pub fn as_gallons_uk(&self) -> f64 {
        self.liters * LITER_GALLONS_UK_FACTOR
    }

    /// Convert Volume to a floating point value in Cubic Feet (ft³)
    pub fn as_cubic_feet(&self) -> f64 {
        self.liters * LITER_CUBIC_FEET_FACTOR
    }

    /// Convert Volume to a floating point value in Cubic Yards (yd³)
    pub fn as_cubic_yards(&self) -> f64 {
        self.liters * LITER_CUBIC_YARD_FACTOR
    }
}

impl Measurement for Volume {
    fn get_base_units(&self) -> f64 {
        self.liters
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_liters(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "l"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("pl", 1e-12),
            ("nl", 1e-9),
            ("\u{00B5}l", 1e-6),
            ("ml", 1e-3),
            ("l", 1e0),
            ("m\u{00B3}", 1e3),
            ("km\u{00B3}", 1e12),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Volume }
