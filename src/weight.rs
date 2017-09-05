use super::measurement::*;

/// The `Weight` struct can be used to deal with weights in a common way.
///
/// #Example
///
/// ```
/// use measurements::Weight;
///
/// let metric_ton = Weight::from_metric_tons(1.0);
/// let united_states_tons = metric_ton.as_short_tons();
/// let united_states_pounds = metric_ton.as_pounds();
/// println!(
///     "One metric ton is {} U.S. tons - that's {} pounds!",
///     united_states_tons, united_states_pounds);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Weight {
    kilograms: f64,
}

impl Weight {
    // Inputs, metric
    pub fn from_kilograms(kilograms: f64) -> Self {
        Weight { kilograms: kilograms }
    }

    pub fn from_micrograms(micrograms: f64) -> Self {
        Self::from_kilograms(micrograms / 1000000000.0)
    }

    pub fn from_milligrams(milligrams: f64) -> Self {
        Self::from_kilograms(milligrams / 1000000.0)
    }

    pub fn from_carats(carats: f64) -> Self {
        Self::from_kilograms(carats / 5000.0)
    }

    pub fn from_grams(grams: f64) -> Self {
        Self::from_kilograms(grams / 1000.0)
    }

    pub fn from_metric_tons(metric_tons: f64) -> Self {
        Self::from_kilograms(metric_tons * 1000.0)
    }

    // Inputs, imperial
    pub fn from_grains(grains: f64) -> Self {
        Self::from_kilograms(grains / 15432.358)
    }

    pub fn from_pennyweights(pennyweights: f64) -> Self {
        Self::from_kilograms(pennyweights / 643.01493)
    }

    pub fn from_ounces(ounces: f64) -> Self {
        Self::from_kilograms(ounces / 35.273962)
    }

    pub fn from_troy_ounces(troy_ounces: f64) -> Self {
        Self::from_kilograms(troy_ounces / 32.150747)
    }

    pub fn from_pounds(pounds: f64) -> Self {
        Self::from_kilograms(pounds / 2.2046228)
    }

    pub fn from_troy_pounds(troy_pounds: f64) -> Self {
        Self::from_kilograms(troy_pounds / 2.6792289)
    }

    pub fn from_stones(stones: f64) -> Self {
        Self::from_kilograms(stones / 0.15747304)
    }

    pub fn from_short_tons(short_tons: f64) -> Self {
        Self::from_kilograms(short_tons * 907.18475)
    }

    pub fn from_long_tons(long_tons: f64) -> Self {
        Self::from_kilograms(long_tons * 1016.0469)
    }

    // Outputs, metric
    pub fn as_micrograms(&self) -> f64 {
        self.kilograms * 1000000000.0
    }

    pub fn as_milligrams(&self) -> f64 {
        self.kilograms * 1000000.0
    }

    pub fn as_carats(&self) -> f64 {
        self.kilograms * 5000.0
    }

    pub fn as_grams(&self) -> f64 {
        self.kilograms * 1000.0
    }

    pub fn as_kilograms(&self) -> f64 {
        self.kilograms
    }

    pub fn as_metric_tons(&self) -> f64 {
        self.kilograms / 1000.0
    }

    // Outputs, imperial
    pub fn as_grains(&self) -> f64 {
        self.kilograms * 15432.358
    }

    pub fn as_pennyweights(&self) -> f64 {
        self.kilograms * 643.01493
    }

    pub fn as_ounces(&self) -> f64 {
        self.kilograms * 35.273962
    }

    pub fn as_pounds(&self) -> f64 {
        self.kilograms * 2.2046228
    }

    pub fn as_troy_ounces(&self) -> f64 {
        self.kilograms * 32.150747
    }

    pub fn as_troy_pounds(&self) -> f64 {
        self.kilograms * 2.6792289
    }

    pub fn as_stones(&self) -> f64 {
        self.kilograms * 0.15747304
    }

    pub fn as_short_tons(&self) -> f64 {
        self.kilograms / 907.18475
    }

    pub fn as_long_tons(&self) -> f64 {
        self.kilograms / 1016.0469
    }
}

impl Measurement for Weight {
    fn get_base_units(&self) -> f64 {
        self.kilograms
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_kilograms(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "kg"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("ng", 1e-12),
            ("\u{00B5}g", 1e-9),
            ("mg", 1e-6),
            ("g", 1e-3),
            ("kg", 1e0),
            ("tonnes", 1e3),
            ("thousand tonnes", 1e6),
            ("million tonnes", 1e9),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Weight }
