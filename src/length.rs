use super::measurement::*;

// Constants, metric
pub const METER_NANOMETER_FACTOR: f64 = 1000000000.0;
pub const METER_MICROMETER_FACTOR: f64 = 1000000.0;
pub const METER_MILLIMETER_FACTOR: f64 = 1000.0;
pub const METER_CENTIMETER_FACTOR: f64 = 100.0;
pub const METER_DECAMETER_FACTOR: f64 = 0.1;
pub const METER_HECTOMETER_FACTOR: f64 = 0.01;
pub const METER_KILOMETER_FACTOR: f64 = 0.001;

// Constants, imperial
pub const METER_INCH_FACTOR: f64 = 39.3700787402;
pub const METER_FEET_FACTOR: f64 = 3.28083989501;
pub const METER_YARD_FACTOR: f64 = 1.09361329834;
pub const METER_FURLONG_FACTOR: f64 = 0.0049709695379;
pub const METER_MILE_FACTOR: f64 = 0.000621371192237;

/// The `Length` struct can be used to deal with lengths in a common way.
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

impl Length {
    // Inputs, metric, with both spellings of meter/metre

    pub fn from_meters(meters: f64) -> Self {
        Length { meters: meters }
    }

    pub fn from_metres(metres: f64) -> Self {
        Self::from_meters(metres)
    }

    pub fn from_nanometers(nanometers: f64) -> Self {
        Self::from_meters(nanometers / METER_NANOMETER_FACTOR)
    }

    pub fn from_nanometres(nanometers: f64) -> Self {
        Self::from_nanometers(nanometers)
    }

    pub fn from_micrometers(micrometers: f64) -> Self {
        Self::from_meters(micrometers / METER_MICROMETER_FACTOR)
    }

    pub fn from_micrometres(micrometers: f64) -> Self {
        Self::from_micrometers(micrometers)
    }

    pub fn from_millimeters(millimeters: f64) -> Self {
        Self::from_meters(millimeters / METER_MILLIMETER_FACTOR)
    }

    pub fn from_millimetres(millimeters: f64) -> Self {
        Self::from_millimeters(millimeters)
    }

    pub fn from_centimeters(centimeters: f64) -> Self {
        Self::from_meters(centimeters / METER_CENTIMETER_FACTOR)
    }

    pub fn from_centimetres(centimeters: f64) -> Self {
        Self::from_centimeters(centimeters)
    }

    pub fn from_decameters(decameters: f64) -> Self {
        Self::from_meters(decameters / METER_DECAMETER_FACTOR)
    }

    pub fn from_decametres(decameters: f64) -> Self {
        Self::from_decameters(decameters)
    }

    pub fn from_hectometers(hectometers: f64) -> Self {
        Self::from_meters(hectometers / METER_HECTOMETER_FACTOR)
    }

    pub fn from_hectometres(hectometers: f64) -> Self {
        Self::from_hectometers(hectometers)
    }

    pub fn from_kilometers(kilometers: f64) -> Self {
        Self::from_meters(kilometers / METER_KILOMETER_FACTOR)
    }

    pub fn from_kilometres(kilometers: f64) -> Self {
        Self::from_kilometers(kilometers)
    }

    // Inputs, imperial
    pub fn from_inches(inches: f64) -> Self {
        Self::from_meters(inches / METER_INCH_FACTOR)
    }

    pub fn from_feet(feet: f64) -> Self {
        Self::from_meters(feet / METER_FEET_FACTOR)
    }

    pub fn from_yards(yards: f64) -> Self {
        Self::from_meters(yards / METER_YARD_FACTOR)
    }

    pub fn from_furlongs(furlongs: f64) -> Self {
        Self::from_meters(furlongs / METER_FURLONG_FACTOR)
    }

    pub fn from_miles(miles: f64) -> Self {
        Self::from_meters(miles / METER_MILE_FACTOR)
    }

    // Outputs, metric
    pub fn as_nanometers(&self) -> f64 {
        self.meters * METER_NANOMETER_FACTOR
    }

    pub fn as_nanometres(&self) -> f64 {
        self.as_nanometers()
    }

    pub fn as_micrometers(&self) -> f64 {
        self.meters * METER_MICROMETER_FACTOR
    }

    pub fn as_micrometres(&self) -> f64 {
        self.as_micrometers()
    }

    pub fn as_millimeters(&self) -> f64 {
        self.meters * METER_MILLIMETER_FACTOR
    }

    pub fn as_millimetres(&self) -> f64 {
        self.as_millimeters()
    }

    pub fn as_centimeters(&self) -> f64 {
        self.meters * METER_CENTIMETER_FACTOR
    }

    pub fn as_centimetres(&self) -> f64 {
        self.as_centimeters()
    }

    pub fn as_meters(&self) -> f64 {
        self.meters
    }

    pub fn as_metres(&self) -> f64 {
        self.as_meters()
    }

    pub fn as_decameters(&self) -> f64 {
        self.meters * METER_DECAMETER_FACTOR
    }

    pub fn as_decametres(&self) -> f64 {
        self.as_decameters()
    }

    pub fn as_hectometers(&self) -> f64 {
        self.meters * METER_HECTOMETER_FACTOR
    }

    pub fn as_hectometres(&self) -> f64 {
        self.as_hectometers()
    }

    pub fn as_kilometers(&self) -> f64 {
        self.meters * METER_KILOMETER_FACTOR
    }

    pub fn as_kilometres(&self) -> f64 {
        self.as_kilometers()
    }

    // Outputs, imperial
    pub fn as_inches(&self) -> f64 {
        self.meters * METER_INCH_FACTOR
    }

    pub fn as_feet(&self) -> f64 {
        self.meters * METER_FEET_FACTOR
    }

    pub fn as_yards(&self) -> f64 {
        self.meters * METER_YARD_FACTOR
    }

    pub fn as_furlongs(&self) -> f64 {
        self.meters * METER_FURLONG_FACTOR
    }

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
