use length::*;
use length::consts::*;
use super::{assert_almost_eq};

// Metric
#[test]
fn nanometers() {
    // Basic tests
    assert_almost_eq(Length::from_meters(1.0).as_nanometers(), METER_NANOMETER_FACTOR);
    assert_almost_eq(Length::from_nanometers(METER_NANOMETER_FACTOR).as_meters(), 1.0);
    
    // Floating-point precision tests
    assert_almost_eq(Length::from_nanometers(1.0).as_nanometers(), Length::from_nanometers(1.0).as_nanometers());
    assert_almost_eq(Length::from_nanometers(0.00025).as_nanometers(), Length::from_nanometers(0.00025).as_nanometers());
}

#[test]
fn micrometers() {
    assert_almost_eq(Length::from_meters(1.0).as_micrometers(), METER_MICROMETER_FACTOR);
    assert_almost_eq(Length::from_micrometers(METER_MICROMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn millimeters() {
    assert_almost_eq(Length::from_meters(1.0).as_millimeters(), METER_MILLIMETER_FACTOR);
    assert_almost_eq(Length::from_millimeters(METER_MILLIMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn centimeters() {
    assert_almost_eq(Length::from_meters(1.0).as_centimeters(), METER_CENTIMETER_FACTOR);
    assert_almost_eq(Length::from_centimeters(METER_CENTIMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn decameter() {
    assert_almost_eq(Length::from_meters(1.0).as_decameters(), METER_DECAMETER_FACTOR);
    assert_almost_eq(Length::from_decameters(METER_DECAMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn hectometer() {
    assert_almost_eq(Length::from_meters(1.0).as_hectometer(), METER_HECTOMETER_FACTOR);
    assert_almost_eq(Length::from_hectometers(METER_HECTOMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn kilometer() {
    assert_almost_eq(Length::from_meters(1.0).as_kilometers(), METER_KILOMETER_FACTOR);
    assert_almost_eq(Length::from_kilometers(METER_KILOMETER_FACTOR).as_meters(), 1.0);
}

// Imperial
#[test]
fn inches() {
    assert_almost_eq(Length::from_meters(1.0).as_inches(), METER_INCH_FACTOR);
    assert_almost_eq(Length::from_inches(METER_INCH_FACTOR).as_meters(), 1.0);
}

#[test]
fn feet() {
    assert_almost_eq(Length::from_meters(1.0).as_feet(), METER_FEET_FACTOR);
    assert_almost_eq(Length::from_feet(METER_FEET_FACTOR).as_meters(), 1.0);
}

#[test]
fn yards() {
    assert_almost_eq(Length::from_meters(1.0).as_yards(), METER_YARD_FACTOR);
    assert_almost_eq(Length::from_yards(METER_YARD_FACTOR).as_meters(), 1.0);
}

#[test]
fn furlongs() {
    assert_almost_eq(Length::from_meters(1.0).as_furlongs(), METER_FURLONG_FACTOR);
    assert_almost_eq(Length::from_furlongs(METER_FURLONG_FACTOR).as_meters(), 1.0);
}

#[test]
fn miles() {
    assert_almost_eq(Length::from_meters(1.0).as_miles(), METER_MILE_FACTOR);
    assert_almost_eq(Length::from_miles(METER_MILE_FACTOR).as_meters(), 1.0);
}

// Traits
#[test]
fn add() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a + b;
    assert_almost_eq(c.as_meters(), 6.0);
}

#[test]
fn sub() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a - b;
    assert_almost_eq(c.as_meters(), -2.0);
}

#[test]
fn mul() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a * b;
    assert_almost_eq(c.as_meters(), 8.0);
}

#[test]
fn div() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a / b;
    assert_almost_eq(c.as_meters(), 0.5);
}

#[test]
fn eq() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
