#[cfg(test)]
mod tests;

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
