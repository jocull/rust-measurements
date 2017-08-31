use super::measurement::*;

// Constants, metric
const METER_NANOMETER_FACTOR: f64 = 1000000000.0;
const METER_MICROMETER_FACTOR: f64 = 1000000.0;
const METER_MILLIMETER_FACTOR: f64 = 1000.0;
const METER_CENTIMETER_FACTOR: f64 = 100.0;
const METER_DECAMETER_FACTOR: f64 = 0.1;
const METER_HECTOMETER_FACTOR: f64 = 0.01;
const METER_KILOMETER_FACTOR: f64 = 0.001;

// Constants, imperial
const METER_INCH_FACTOR: f64 = 39.3700787402;
const METER_FEET_FACTOR: f64 = 3.28083989501;
const METER_YARD_FACTOR: f64 = 1.09361329834;
const METER_FURLONG_FACTOR: f64 = 0.0049709695379;
const METER_MILE_FACTOR: f64 = 0.000621371192237;

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
    // Inputs, metric
    pub fn from_meters(meters: f64) -> Self {
        Length { meters: meters }
    }

    pub fn from_nanometers(nanometers: f64) -> Self {
        Self::from_meters(nanometers / METER_NANOMETER_FACTOR)
    }

    pub fn from_micrometers(micrometers: f64) -> Self {
        Self::from_meters(micrometers / METER_MICROMETER_FACTOR)
    }

    pub fn from_millimeters(millimeters: f64) -> Self {
        Self::from_meters(millimeters / METER_MILLIMETER_FACTOR)
    }

    pub fn from_centimeters(centimeters: f64) -> Self {
        Self::from_meters(centimeters / METER_CENTIMETER_FACTOR)
    }

    pub fn from_decameters(decameters: f64) -> Self {
        Self::from_meters(decameters / METER_DECAMETER_FACTOR)
    }

    pub fn from_hectometers(hectometers: f64) -> Self {
        Self::from_meters(hectometers / METER_HECTOMETER_FACTOR)
    }

    pub fn from_kilometers(kilometers: f64) -> Self {
        Self::from_meters(kilometers / METER_KILOMETER_FACTOR)
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

    pub fn as_micrometers(&self) -> f64 {
        self.meters * METER_MICROMETER_FACTOR
    }

    pub fn as_millimeters(&self) -> f64 {
        self.meters * METER_MILLIMETER_FACTOR
    }

    pub fn as_centimeters(&self) -> f64 {
        self.meters * METER_CENTIMETER_FACTOR
    }

    pub fn as_meters(&self) -> f64 {
        self.meters
    }

    pub fn as_decameters(&self) -> f64 {
        self.meters * METER_DECAMETER_FACTOR
    }

    pub fn as_hectometer(&self) -> f64 {
        self.meters * METER_HECTOMETER_FACTOR
    }

    pub fn as_kilometers(&self) -> f64 {
        self.meters * METER_KILOMETER_FACTOR
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
}

implement_measurement! { Length }

impl ::std::fmt::Display for Length {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let p = f.precision().unwrap_or(1);
        let w = f.width().unwrap_or(0);
        write!(f, "{value:width$.prec$}\u{00A0}m", prec=p, width=w, value=self.as_meters())
    }
}
