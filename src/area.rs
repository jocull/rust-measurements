use super::measurement::*;
use super::length;

const SQUARE_METER_ACRE_FACTOR: f64 = 1.0 / 4046.86;

/// The `Area` struct can be used to deal with areas in a common way.
/// Common metric and imperial units are supported.
///
/// # Example
///
/// ```
/// use measurements::Area;
///
/// let football_field = Area::from_square_meters(7140.0);
/// let acres = football_field.as_acres();
/// println!("There are {} acres in a football field.", acres);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Area {
    square_meters: f64,
}

impl Area {
    // Inputs, metric, with both spellings of meter/metre

    pub fn from_square_meters(square_meters: f64) -> Self {
        Area { square_meters: square_meters }
    }

    pub fn from_square_metres(square_metres: f64) -> Self {
        Self::from_square_meters(square_metres)
    }

    pub fn from_square_nanometers(square_nanometers: f64) -> Self {
        Self::from_square_meters(
            square_nanometers / (length::METER_NANOMETER_FACTOR * length::METER_NANOMETER_FACTOR),
        )
    }

    pub fn from_square_nanometres(square_nanometres: f64) -> Self {
        Self::from_square_nanometers(square_nanometres)
    }

    pub fn from_square_micrometers(square_micrometers: f64) -> Self {
        Self::from_square_meters(
            square_micrometers / (length::METER_MICROMETER_FACTOR * length::METER_MICROMETER_FACTOR),
        )
    }

    pub fn from_square_micrometres(square_micrometres: f64) -> Self {
        Self::from_square_micrometers(square_micrometres)
    }

    pub fn from_square_millimeters(square_millimeters: f64) -> Self {
        Self::from_square_meters(
            square_millimeters / (length::METER_MILLIMETER_FACTOR * length::METER_MILLIMETER_FACTOR),
        )
    }

    pub fn from_square_millimetres(square_millimetres: f64) -> Self {
        Self::from_square_millimeters(square_millimetres)
    }

    pub fn from_square_centimeters(square_centimeters: f64) -> Self {
        Self::from_square_meters(
            square_centimeters / (length::METER_CENTIMETER_FACTOR * length::METER_CENTIMETER_FACTOR),
        )
    }

    pub fn from_square_centimetres(square_centimetres: f64) -> Self {
        Self::from_square_centimeters(square_centimetres)
    }

    pub fn from_square_decimeters(square_decimeters: f64) -> Self {
        Self::from_square_meters(
            square_decimeters / (length::METER_DECIMETER_FACTOR * length::METER_DECIMETER_FACTOR),
        )
    }

    pub fn from_square_decimetres(square_decimetres: f64) -> Self {
        Self::from_square_decimeters(square_decimetres)
    }

    pub fn from_square_hectometers(square_hectometers: f64) -> Self {
        Self::from_square_meters(
            square_hectometers / (length::METER_HECTOMETER_FACTOR * length::METER_HECTOMETER_FACTOR),
        )
    }

    pub fn from_square_hectometres(square_hectometres: f64) -> Self {
        Self::from_square_hectometers(square_hectometres)
    }

    pub fn from_hectares(hectares: f64) -> Self {
        Self::from_square_hectometers(hectares)
    }

    pub fn from_square_kilometers(square_kilometers: f64) -> Self {
        Self::from_square_meters(
            square_kilometers / (length::METER_KILOMETER_FACTOR * length::METER_KILOMETER_FACTOR),
        )
    }

    pub fn from_square_kilometres(square_kilometres: f64) -> Self {
        Self::from_square_kilometers(square_kilometres)
    }

    // Inputs, imperial
    pub fn from_square_inches(square_inches: f64) -> Self {
        Self::from_square_meters(
            square_inches / (length::METER_INCH_FACTOR * length::METER_INCH_FACTOR),
        )
    }

    pub fn from_square_feet(square_feet: f64) -> Self {
        Self::from_square_meters(
            square_feet / (length::METER_FEET_FACTOR * length::METER_FEET_FACTOR),
        )
    }

    pub fn from_square_yards(square_yards: f64) -> Self {
        Self::from_square_meters(
            square_yards / (length::METER_YARD_FACTOR * length::METER_YARD_FACTOR),
        )
    }

    pub fn from_acres(acres: f64) -> Self {
        Self::from_square_meters(acres / SQUARE_METER_ACRE_FACTOR)
    }

    pub fn from_square_miles(square_miles: f64) -> Self {
        Self::from_square_meters(
            square_miles / (length::METER_MILE_FACTOR * length::METER_MILE_FACTOR),
        )
    }

    // Outputs, metric
    pub fn as_square_nanometers(&self) -> f64 {
        self.square_meters * (length::METER_NANOMETER_FACTOR * length::METER_NANOMETER_FACTOR)
    }

    pub fn as_square_nanometres(&self) -> f64 {
        self.as_square_nanometers()
    }

    pub fn as_square_micrometers(&self) -> f64 {
        self.square_meters * (length::METER_MICROMETER_FACTOR * length::METER_MICROMETER_FACTOR)
    }

    pub fn as_square_micrometres(&self) -> f64 {
        self.as_square_micrometers()
    }

    pub fn as_square_millimeters(&self) -> f64 {
        self.square_meters * (length::METER_MILLIMETER_FACTOR * length::METER_MILLIMETER_FACTOR)
    }

    pub fn as_square_millimetres(&self) -> f64 {
        self.as_square_millimeters()
    }

    pub fn as_square_centimeters(&self) -> f64 {
        self.square_meters * (length::METER_CENTIMETER_FACTOR * length::METER_CENTIMETER_FACTOR)
    }

    pub fn as_square_centimetres(&self) -> f64 {
        self.as_square_centimeters()
    }

    pub fn as_square_meters(&self) -> f64 {
        self.square_meters
    }

    pub fn as_square_metres(&self) -> f64 {
        self.as_square_meters()
    }

    pub fn as_square_decimeters(&self) -> f64 {
        self.square_meters * (length::METER_DECIMETER_FACTOR * length::METER_DECIMETER_FACTOR)
    }

    pub fn as_square_decimetres(&self) -> f64 {
        self.as_square_decimeters()
    }

    pub fn as_square_hectometers(&self) -> f64 {
        self.square_meters * (length::METER_HECTOMETER_FACTOR * length::METER_HECTOMETER_FACTOR)
    }

    pub fn as_square_hectometres(&self) -> f64 {
        self.as_square_hectometers()
    }

    pub fn as_hectares(&self) -> f64 {
        self.as_square_hectometers()
    }

    pub fn as_square_kilometers(&self) -> f64 {
        self.square_meters * (length::METER_KILOMETER_FACTOR * length::METER_KILOMETER_FACTOR)
    }

    pub fn as_square_kilometres(&self) -> f64 {
        self.as_square_kilometers()
    }

    // Outputs, imperial
    pub fn as_square_inches(&self) -> f64 {
        self.square_meters * (length::METER_INCH_FACTOR * length::METER_INCH_FACTOR)
    }

    pub fn as_square_feet(&self) -> f64 {
        self.square_meters * (length::METER_FEET_FACTOR * length::METER_FEET_FACTOR)
    }

    pub fn as_square_yards(&self) -> f64 {
        self.square_meters * (length::METER_YARD_FACTOR * length::METER_YARD_FACTOR)
    }

    pub fn as_acres(&self) -> f64 {
        self.square_meters * SQUARE_METER_ACRE_FACTOR
    }

    pub fn as_square_miles(&self) -> f64 {
        self.square_meters * (length::METER_MILE_FACTOR * length::METER_MILE_FACTOR)
    }
}

impl Measurement for Area {
    fn get_base_units(&self) -> f64 {
        self.square_meters
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_square_meters(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "m\u{00B2}"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("nm\u{00B2}", 1e-18),
            ("\u{00B5}m\u{00B2}", 1e-12),
            ("mm\u{00B2}", 1e-6),
            ("cm\u{00B2}", 1e-4),
            ("m\u{00B2}", 1e0),
            ("km\u{00B2}", 1e6),
            ("thousand km\u{00B2}", 1e9),
            ("million km\u{00B2}", 1e12),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Area }
