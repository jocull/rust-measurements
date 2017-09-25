//! Types and contants for handling lengths (or distances).

use super::measurement::*;

// Constants, metric

/// Number of nanometers in a meter
pub const METER_NANOMETER_FACTOR: f64 = 1000000000.0;
/// Number of µm in a meter
pub const METER_MICROMETER_FACTOR: f64 = 1000000.0;
/// Number of mm in a meter
pub const METER_MILLIMETER_FACTOR: f64 = 1000.0;
/// Number of cm in a meter
pub const METER_CENTIMETER_FACTOR: f64 = 100.0;
/// Number of dm in a meter
pub const METER_DECIMETER_FACTOR: f64 = 10.0;
/// Number of hm in a meter
pub const METER_HECTOMETER_FACTOR: f64 = 0.01;
/// Number of km in a meter
pub const METER_KILOMETER_FACTOR: f64 = 0.001;

// Constants, imperial

/// Number of inches in a meter
pub const METER_INCH_FACTOR: f64 = 1000.0 / 25.4;
/// Number of feet in a meter
pub const METER_FEET_FACTOR: f64 = 1000.0 / (25.4 * 12.0);
/// Number of yards in a meter
pub const METER_YARD_FACTOR: f64 = 1000.0 / (25.4 * 12.0 * 3.0);
/// Number of furlongs in a meter
pub const METER_FURLONG_FACTOR: f64 = 1000.0 / (25.4 * 12.0 * 3.0 * 220.0);
/// Number of miles in a meter
pub const METER_MILE_FACTOR: f64 = 1000.0 / (25.4 * 12.0 * 3.0 * 1760.0);

/// The Length struct can be used to deal with lengths in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::Length;
///
/// let football_field = Length::from_yards(100.0);
/// let meters = football_field.as_meters();
/// println!("There are {} meters in a football field.", meters);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Length {
    meters: f64,
}

/// Distance is a synonym for Length
pub type Distance = Length;

impl Length {
    /// Create a new Length from a floating point value in meters
    pub fn from_meters(meters: f64) -> Self {
        Length { meters: meters }
    }

    /// Create a new Length from a floating point value in metres.
    pub fn from_metres(metres: f64) -> Self {
        Self::from_meters(metres)
    }

    /// Create a new Length from a floating point value in nanometers.
    pub fn from_nanometers(nanometers: f64) -> Self {
        Self::from_meters(nanometers / METER_NANOMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in nanometres.
    pub fn from_nanometres(nanometers: f64) -> Self {
        Self::from_nanometers(nanometers)
    }

    /// Create a new Length from a floating point value in micrometers.
    pub fn from_micrometers(micrometers: f64) -> Self {
        Self::from_meters(micrometers / METER_MICROMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in micrometres.
    pub fn from_micrometres(micrometers: f64) -> Self {
        Self::from_micrometers(micrometers)
    }

    /// Create a new Length from a floating point value in millimeters.
    pub fn from_millimeters(millimeters: f64) -> Self {
        Self::from_meters(millimeters / METER_MILLIMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in millimetres.
    pub fn from_millimetres(millimeters: f64) -> Self {
        Self::from_millimeters(millimeters)
    }

    /// Create a new Length from a floating point value in centimeters.
    pub fn from_centimeters(centimeters: f64) -> Self {
        Self::from_meters(centimeters / METER_CENTIMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in centimetres.
    pub fn from_centimetres(centimeters: f64) -> Self {
        Self::from_centimeters(centimeters)
    }

    /// Create a new Length from a floating point value in decimeters.
    pub fn from_decimeters(decimeters: f64) -> Self {
        Self::from_meters(decimeters / METER_DECIMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in decimetres.
    pub fn from_decimetres(decimeters: f64) -> Self {
        Self::from_decimeters(decimeters)
    }

    /// Create a new Length from a floating point value in hectometers.
    pub fn from_hectometers(hectometers: f64) -> Self {
        Self::from_meters(hectometers / METER_HECTOMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in hectometres.
    pub fn from_hectometres(hectometers: f64) -> Self {
        Self::from_hectometers(hectometers)
    }

    /// Create a new Length from a floating point value in kilometers.
    pub fn from_kilometers(kilometers: f64) -> Self {
        Self::from_meters(kilometers / METER_KILOMETER_FACTOR)
    }

    /// Create a new Length from a floating point value in kilometres.
    pub fn from_kilometres(kilometers: f64) -> Self {
        Self::from_kilometers(kilometers)
    }

    /// Create a new Length from a floating point value in inches.
    pub fn from_inches(inches: f64) -> Self {
        Self::from_meters(inches / METER_INCH_FACTOR)
    }

    /// Create a new Length from a floating point value in feet.
    pub fn from_feet(feet: f64) -> Self {
        Self::from_meters(feet / METER_FEET_FACTOR)
    }

    /// Create a new Length from a floating point value in yards.
    pub fn from_yards(yards: f64) -> Self {
        Self::from_meters(yards / METER_YARD_FACTOR)
    }

    /// Create a new Length from a floating point value in furlongs.
    pub fn from_furlongs(furlongs: f64) -> Self {
        Self::from_meters(furlongs / METER_FURLONG_FACTOR)
    }

    /// Create a new Length from a floating point value in miles.
    pub fn from_miles(miles: f64) -> Self {
        Self::from_meters(miles / METER_MILE_FACTOR)
    }

    /// Convert this Length to a floating point value in nanometers
    pub fn as_nanometers(&self) -> f64 {
        self.meters * METER_NANOMETER_FACTOR
    }

    /// Convert this Length to a floating point value in nanometres
    pub fn as_nanometres(&self) -> f64 {
        self.as_nanometers()
    }

    /// Convert this Length to a floating point value in micrometers
    pub fn as_micrometers(&self) -> f64 {
        self.meters * METER_MICROMETER_FACTOR
    }

    /// Convert this Length to a floating point value in micrometres
    pub fn as_micrometres(&self) -> f64 {
        self.as_micrometers()
    }

    /// Convert this Length to a floating point value in millimeters
    pub fn as_millimeters(&self) -> f64 {
        self.meters * METER_MILLIMETER_FACTOR
    }

    /// Convert this Length to a floating point value in millimetres
    pub fn as_millimetres(&self) -> f64 {
        self.as_millimeters()
    }

    /// Convert this Length to a floating point value in centimeters
    pub fn as_centimeters(&self) -> f64 {
        self.meters * METER_CENTIMETER_FACTOR
    }

    /// Convert this Length to a floating point value in centimetres
    pub fn as_centimetres(&self) -> f64 {
        self.as_centimeters()
    }

    /// Convert this Length to a floating point value in meters
    pub fn as_meters(&self) -> f64 {
        self.meters
    }

    /// Convert this Length to a floating point value in metres
    pub fn as_metres(&self) -> f64 {
        self.as_meters()
    }

    /// Convert this Length to a floating point value in decimeters
    pub fn as_decimeters(&self) -> f64 {
        self.meters * METER_DECIMETER_FACTOR
    }

    /// Convert this Length to a floating point value in decimetres
    pub fn as_decimetres(&self) -> f64 {
        self.as_decimeters()
    }

    /// Convert this Length to a floating point value in hectometers
    pub fn as_hectometers(&self) -> f64 {
        self.meters * METER_HECTOMETER_FACTOR
    }

    /// Convert this Length to a floating point value in hectometres
    pub fn as_hectometres(&self) -> f64 {
        self.as_hectometers()
    }

    /// Convert this Length to a floating point value in kilometers
    pub fn as_kilometers(&self) -> f64 {
        self.meters * METER_KILOMETER_FACTOR
    }

    /// Convert this Length to a floating point value in kilometres
    pub fn as_kilometres(&self) -> f64 {
        self.as_kilometers()
    }

    /// Convert this Length to a floating point value in inches
    pub fn as_inches(&self) -> f64 {
        self.meters * METER_INCH_FACTOR
    }

    /// Convert this Length to a floating point value in feet
    pub fn as_feet(&self) -> f64 {
        self.meters * METER_FEET_FACTOR
    }

    /// Convert this Length to a floating point value in yards
    pub fn as_yards(&self) -> f64 {
        self.meters * METER_YARD_FACTOR
    }

    /// Convert this Length to a floating point value in furlongs
    pub fn as_furlongs(&self) -> f64 {
        self.meters * METER_FURLONG_FACTOR
    }

    /// Convert this Length to a floating point value in miles
    pub fn as_miles(&self) -> f64 {
        self.meters * METER_MILE_FACTOR
    }
}

impl Measurement for Length {
    fn get_base_units(&self) -> f64 {
        self.meters
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_meters(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "m"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("pm", 1e-12),
            ("nm", 1e-9),
            ("\u{00B5}m", 1e-6),
            ("mm", 1e-3),
            ("cm", 1e-2),
            ("m", 1e0),
            ("km", 1e3),
            ("thousand km", 1e6),
            ("million km", 1e9),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Length }
