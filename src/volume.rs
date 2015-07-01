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
    litres: f64
}

// Constants, metric
const LITER_MILLILITERS_FACTOR: f64 = 1000.0;
const LITER_CUBIC_CENTIMETER_FACTOR: f64 = 1000.0;
const LITER_CUBIC_METER_FACTOR: f64 = 0.001;

// Constants, imperial
const LITER_DROP_FACTOR: f64 = 15419.6298055;
const LITER_DRAM_FACTOR: f64 = 270.510351863;
const LITER_TEASPOONS_FACTOR: f64 = 202.8841362;
const LITER_TABLESPOONS_FACTOR: f64 = 67.6280454;
const LITER_CUBIC_INCHES_FACTOR: f64 = 61.0237440947;
const LITER_FLUID_OUNCES_UK_FACTOR: f64 = 35.19506424;
const LITER_FLUID_OUNCES_FACTOR: f64 = 33.8140227;
const LITER_CUP_FACTOR: f64 = 4.226752838;
const LITER_PINTS_FACTOR: f64 = 2.11337641887;
const LITER_PINTS_UK_FACTOR: f64 = 1.75975398639;
const LITER_QUARTS_FACTOR: f64 = 1.05668820943;
const LITER_GALLONS_FACTOR: f64 = 0.264172052358;
const LITER_GALLONS_UK_FACTOR: f64 = 0.219969248299;
const LITER_CUBIC_FEET_FACTOR: f64 = 0.0353146667215;
const LITER_CUBIC_YARD_FACTOR: f64 = 0.0013079506193;

impl Volume {
    // Inputs, metric
    pub fn from_litres(litres: f64) -> Self {
        Volume { litres: litres }
    }
    
    pub fn from_cubic_centimeters(cubic_centimeters: f64) -> Self {
        Self::from_litres(cubic_centimeters / LITER_CUBIC_CENTIMETER_FACTOR)
    }
    
    pub fn from_milliliters(milliliters: f64) -> Self {
        Self::from_litres(milliliters / LITER_MILLILITERS_FACTOR)
    }
        
    pub fn from_cubic_meters(cubic_meters: f64) -> Self {
        Self::from_litres(cubic_meters / LITER_CUBIC_METER_FACTOR)
    }
    
    // Inputs, imperial
    pub fn from_drops(drops: f64) -> Self {
        Self::from_litres(drops / LITER_DROP_FACTOR)
    }
    
    pub fn from_drams(drams: f64) -> Self {
        Self::from_litres(drams / LITER_DRAM_FACTOR)
    }
    
    pub fn from_teaspoons(teaspoons: f64) -> Self {
        Self::from_litres(teaspoons / LITER_TEASPOONS_FACTOR)
    }

    pub fn from_tablespoons(tablespoons: f64) -> Self {
        Self::from_litres(tablespoons / LITER_TABLESPOONS_FACTOR)
    }
    
    pub fn from_fluid_ounces_uk(fluid_ounces_uk: f64) -> Self {
        Self::from_litres(fluid_ounces_uk / LITER_FLUID_OUNCES_UK_FACTOR)
    }

    pub fn from_fluid_ounces(fluid_ounces: f64) -> Self {
        Self::from_litres(fluid_ounces / LITER_FLUID_OUNCES_FACTOR)
    }
    
    pub fn from_cubic_inches(cubic_inches: f64) -> Self {
        Self::from_litres(cubic_inches / LITER_CUBIC_INCHES_FACTOR)
    }
    
    pub fn from_cups(cups: f64) -> Self {
        Self::from_litres(cups / LITER_CUP_FACTOR)
    }
    
    pub fn from_pints(pints: f64) -> Self {
        Self::from_litres(pints / LITER_PINTS_FACTOR)
    }

    pub fn from_pints_uk(pints_uk: f64) -> Self {
        Self::from_litres(pints_uk / LITER_PINTS_UK_FACTOR)
    }

    pub fn from_quarts(quarts: f64) -> Self {
        Self::from_litres(quarts / LITER_QUARTS_FACTOR)
    }
    
    pub fn from_gallons(gallons: f64) -> Self {
        Self::from_litres(gallons / LITER_GALLONS_FACTOR)
    }
    
    pub fn from_gallons_uk(gallons_uk: f64) -> Self {
        Self::from_litres(gallons_uk / LITER_GALLONS_UK_FACTOR)
    }
    
    pub fn from_cubic_feet(cubic_feet: f64) -> Self {
        Self::from_litres(cubic_feet / LITER_CUBIC_FEET_FACTOR)
    }
    
    pub fn from_cubic_yards(cubic_yards: f64) -> Self {
        Self::from_litres(cubic_yards / LITER_CUBIC_YARD_FACTOR)
    }
    
    // Outputs, metric
    pub fn as_cubic_centimeters(&self) -> f64 {
        self.litres * LITER_CUBIC_CENTIMETER_FACTOR
    }
    
    pub fn as_milliliters(&self) -> f64 {
        self.litres * LITER_MILLILITERS_FACTOR
    }
    
    pub fn as_litres(&self) -> f64 {
        self.litres
    }
    
    pub fn as_cubic_meters(&self) -> f64 {
        self.litres * LITER_CUBIC_METER_FACTOR
    }
    
    // Outputs, imperial
    pub fn as_drops(&self) -> f64 {
        self.litres * LITER_DROP_FACTOR
    }
    
    pub fn as_drams(&self) -> f64 {
        self.litres * LITER_DRAM_FACTOR
    }
    
    pub fn as_teaspoons(&self) -> f64 {
        self.litres * LITER_TEASPOONS_FACTOR
    }

    pub fn as_tablespoons(&self) -> f64 {
        self.litres * LITER_TABLESPOONS_FACTOR
    }
    
    pub fn as_cubic_inches(&self) -> f64 {
        self.litres * LITER_CUBIC_INCHES_FACTOR
    }
    
    pub fn as_fluid_ounces_uk(&self) -> f64 {
        self.litres * LITER_FLUID_OUNCES_UK_FACTOR
    }

    pub fn as_fluid_ounces(&self) -> f64 {
        self.litres * LITER_FLUID_OUNCES_FACTOR
    }
    
    pub fn as_cups(&self) -> f64 {
        self.litres * LITER_CUP_FACTOR
    }
    
    pub fn as_pints(&self) -> f64 {
        self.litres * LITER_PINTS_FACTOR
    }

    pub fn as_pints_uk(&self) -> f64 {
        self.litres * LITER_PINTS_UK_FACTOR
    }

    pub fn as_quarts(&self) -> f64 {
        self.litres * LITER_QUARTS_FACTOR
    }
    
    pub fn as_gallons(&self) -> f64 {
        self.litres * LITER_GALLONS_FACTOR
    }
    
    pub fn as_gallons_uk(&self) -> f64 {
        self.litres * LITER_GALLONS_UK_FACTOR
    }
    
    pub fn as_cubic_feet(&self) -> f64 {
        self.litres * LITER_CUBIC_FEET_FACTOR
    }
    
    pub fn as_cubic_yards(&self) -> f64 {
        self.litres * LITER_CUBIC_YARD_FACTOR
    }
}

impl Measurement for Volume {
    fn get_base_units(&self) -> f64 {
        self.litres
    }
    
    fn from_base_units(units: f64) -> Self {
        Self::from_litres(units)
    }
}

implement_measurement! { Volume }
