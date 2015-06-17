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
