pub mod consts;
mod traits;
use self::consts::*;

/// The `Length` struct can be used to deal with lengths in a common way.
/// Common metric and imperial units are supported.
#[derive(Copy, Clone, Debug)]
pub struct Length {
    meters: f64
}

impl Length {
    // Inputs, metric
    pub fn from_meters(meters: f64) -> Length {
        Length { meters: meters }
    }
    
    pub fn from_nanometers(nanometers: f64) -> Length {
        Self::from_meters(nanometers / METER_NANOMETER_FACTOR)
    }
    
    pub fn from_micrometers(micrometers: f64) -> Length {
        Self::from_meters(micrometers / METER_MICROMETER_FACTOR)
    }
    
    pub fn from_millimeters(millimeters: f64) -> Length {
        Self::from_meters(millimeters / METER_MILLIMETER_FACTOR)
    }
    
    pub fn from_centimeters(centimeters: f64) -> Length {
        Self::from_meters(centimeters / METER_CENTIMETER_FACTOR)
    }
    
    pub fn from_decameters(decameters: f64) -> Length {
        Self::from_meters(decameters / METER_DECAMETER_FACTOR)
    }
    
    pub fn from_hectometers(hectometers: f64) -> Length {
        Self::from_meters(hectometers / METER_HECTOMETER_FACTOR)
    }
    
    pub fn from_kilometers(kilometers: f64) -> Length {
        Self::from_meters(kilometers / METER_KILOMETER_FACTOR)
    }
    
    // Inputs, imperial
    pub fn from_inches(inches: f64) -> Length {
        Self::from_meters(inches / METER_INCH_FACTOR)
    }
    
    pub fn from_feet(feet: f64) -> Length {
        Self::from_meters(feet / METER_FEET_FACTOR)
    }
    
    pub fn from_yards(yards: f64) -> Length {
        Self::from_meters(yards / METER_YARD_FACTOR)
    }
    
    pub fn from_furlongs(furlongs: f64) -> Length {
        Self::from_meters(furlongs / METER_FURLONG_FACTOR)
    }
    
    pub fn from_miles(miles: f64) -> Length {
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
