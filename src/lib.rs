#[macro_use]
mod measurement;
pub use measurement::Measurement;

pub mod length;
pub use length::Length;

pub mod temperature;
pub use temperature::{Temperature, TemperatureDelta};

pub mod mass;
pub use mass::Mass;

pub mod volume;
pub use volume::Volume;

pub mod pressure;
pub use pressure::Pressure;

pub mod speed;
pub use speed::Speed;

pub mod acceleration;
pub use acceleration::Acceleration;

pub mod energy;
pub use energy::Energy;

pub mod power;
pub use power::Power;

pub mod force;
pub use force::Force;

pub mod area;
pub use area::Area;

pub fn duration_as_f64(duration: std::time::Duration) -> f64 {
    duration.as_secs() as f64 + ((duration.subsec_nanos() as f64) * 1e-9)
}

// Include when running tests, but don't export them
#[cfg(test)]
#[allow(dead_code)]
mod tests;
