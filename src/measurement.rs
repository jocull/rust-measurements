/// The `Measurement` trait and the `implement_measurement!` macro
/// provides a common way for various measurements to be implemented.
///
/// # Example
/// ```
/// // Importing the `implement_measurement` macro from the external crate is important
/// #[macro_use]
/// extern crate measurements;
///
/// use measurements::Measurement;
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
///
///    fn get_base_units_name(&self) -> &'static str {
///        "cu"
///    }
/// }
///
/// // Invoke the macro to automatically implement Add, Sub, etc...
/// implement_measurement! { Cubits }
///
/// // The main function here is only included to make doc tests compile.
/// // You should't need it in your own code.
/// fn main() { }
/// ```
pub trait Measurement {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        (self.get_base_units_name(), self.get_base_units())
    }

    /// Given a list of units and their scale relative to the base unit,
    /// select the most appropriate one.
    ///
    /// The list must be smallest to largest, e.g. ("nanometre", 10-9) to
    /// ("kilometre", 10e3)
    fn pick_appropriate_units(&self, list: &[(&'static str, f64)]) -> (&'static str, f64) {
        for &(ref unit, ref scale) in list.iter().rev() {
            let value = self.get_base_units() / scale;
            if value.abs() > 1.0 {
                return (unit, value);
            }
        }
        (list[0].0, self.get_base_units() / list[0].1)
    }

    fn get_base_units_name(&self) -> &'static str;

    fn get_base_units(&self) -> f64;

    fn from_base_units(units: f64) -> Self;
}

/// This is a special macro that creates the code to implement
/// std::fmt::Display.
#[macro_export]
macro_rules! implement_display {
    ($($t:ty)*) => ($(

        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let (unit, value) = self.get_appropriate_units();
                value.fmt(f)?;
                write!(f, "\u{00A0}{0}", unit)
            }
        }
    )*)
}


/// This is a special macro that creates the code to implement
/// operator and comparison overrides.
#[macro_export]
macro_rules! implement_measurement {
    ($($t:ty)*) => ($(

        implement_display!( $t );

        impl ::std::ops::Add for $t {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::from_base_units(self.get_base_units() + rhs.get_base_units())
            }
        }

        impl ::std::ops::Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::from_base_units(self.get_base_units() - rhs.get_base_units())
            }
        }

        // Dividing a `$t` by another `$` returns a ratio.
        //
        impl ::std::ops::Div<$t> for $t {
            type Output = f64;

            fn div(self, rhs: Self) -> f64 {
                self.get_base_units() / rhs.get_base_units()
            }
        }

        // Dividing a `$` by a factor returns a new portion of the measurement.
        //
        impl ::std::ops::Div<f64> for $t {
            type Output = Self;

            fn div(self, rhs: f64) -> Self {
                Self::from_base_units(self.get_base_units() / rhs)
            }
        }

        // Multiplying a `$t` by a factor increases (or decreases) that
        // measurement a number of times.
        impl ::std::ops::Mul<f64> for $t {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self {
                Self::from_base_units(self.get_base_units() * rhs)
            }
        }

        // Multiplying by a factor is commutative
        impl ::std::ops::Mul<$t> for f64 {
            type Output = $t;

            fn mul(self, rhs: $t) -> $t {
                rhs * self
            }
        }

        impl ::std::cmp::Eq for $t { }
        impl ::std::cmp::PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.get_base_units() == other.get_base_units()
            }
        }

        impl ::std::cmp::PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.get_base_units().partial_cmp(&other.get_base_units())
            }
        }
    )*)
}
