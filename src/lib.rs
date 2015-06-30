/// The `Measurement` trait and the `implement_measurement!` macro
/// provides a common way for various measurements to be implemented.
///
/// # Example
/// ```
/// // Importing the `implement_measurement` macro from the external crate is important
/// #[macro_use]
/// extern crate measurements;
/// 
/// use measurements::measurement::*;
/// 
/// struct Cubits {
///     forearms: f64
/// }
/// 
/// impl Measurement for Cubits {
///     fn get_base_units(&self) -> f64 {
///         self.forearms
///     }
///     
///     fn from_base_units(units: f64) -> Self {
///         Cubits { forearms: units }
///     }
/// }
///
/// // Invoke the macro to automatically implement Add, Sub, etc...
/// implement_measurement! { Cubits }
///
/// // The main function here is only included to make doc tests compile.
/// // You should't need it in your own code.
/// fn main() { }
/// ```
#[macro_use]
pub mod measurement;

/// The `Length` struct can be used to deal with lengths in a common way.
/// Common metric and imperial units are supported.
/// 
/// # Example
/// 
/// ```
/// use measurements::length::Length;
/// 
/// let football_field = Length::from_yards(100.0);
/// let meters = football_field.as_meters();
/// println!("There are {} meters in a football field.", meters);
/// ```
#[allow(dead_code)]
pub mod length;

/// The `Temperature` struct can be used to deal with temperatures in a common way.
/// 
/// # Example
/// 
/// ```
/// use measurements::temperature::Temperature;
/// 
/// let boiling_water = Temperature::from_celsius(100.0);
/// let fahrenheit = boiling_water.as_fahrenheit();
/// println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);
/// ```
#[allow(dead_code)]
pub mod temperature;

/// TODO: Provide docs and examples for this.
/// 
#[allow(dead_code)]
pub mod weight;

// Include when running tests, but don't export them
#[cfg(test)]
#[allow(dead_code)]
mod tests;
