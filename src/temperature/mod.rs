mod traits;

/// The `Temperature` struct can be used to deal with temperatures in a common way.
#[derive(Copy, Clone, Debug)]
pub struct Temperature {
    kelvin: f64
}

impl Temperature {
    pub fn from_kelvin(kelvin: f64) -> Temperature {
        Temperature { kelvin: kelvin }
    }
    
    pub fn from_celsius(celsius: f64) -> Temperature {
        Self::from_kelvin(celsius + 273.15)
    }
    
    pub fn from_fahrenheit(fahrenheit: f64) -> Temperature {
        Self::from_kelvin((fahrenheit - 32.0) / 1.8 + 273.15)
    }
    
    pub fn from_rankine(rankine: f64) -> Temperature {
        Self::from_kelvin((rankine - 491.67) / 1.8 + 273.15)
    }
    
    pub fn as_kelvin(&self) -> f64 {
        self.kelvin
    }
    
    pub fn as_celsius(&self) -> f64 {
        self.kelvin - 273.15
    }
    
    pub fn as_fahrenheit(&self) -> f64 {
        (self.kelvin - 273.15) * 1.8 + 32.0
    }
    
    pub fn as_rankine(&self) -> f64 {
        (self.kelvin - 273.15) * 1.8 + 491.67
    }
}
